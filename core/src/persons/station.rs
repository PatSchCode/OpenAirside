//! ADR-054, step 3.2: persisted station phases and derived FIFO/occupancy.
//! Only the persons module owns these records; Build remains read-only.
use super::*;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(super) enum Place {
    Entry,
    Queue(u8),
    Service,
    Exit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(super) enum Phase {
    None,
    Approach,
    Backlog,
    Queue { from: Place, to: Place },
    ToService,
    Serving { remaining: u16 },
    Leaving,
    Complete,
}

impl Phase {
    pub(super) fn hash_into(self, h: &mut StateHasher) {
        let (tag, a, b, rest) = match self {
            Self::None => (0, 0, 0, 0),
            Self::Approach => (1, 0, 0, 0),
            Self::Backlog => (2, 0, 0, 0),
            Self::Queue { from, to } => (3, place_code(from), place_code(to), 0),
            Self::ToService => (4, 0, 0, 0),
            Self::Serving { remaining } => (5, 0, 0, remaining),
            Self::Leaving => (6, 0, 0, 0),
            Self::Complete => (7, 0, 0, 0),
        };
        h.bytes(&[tag]);
        h.u32(a);
        h.u32(b);
        h.u32(u32::from(rest));
    }
    fn edge(self) -> Option<(Place, Place)> {
        match self {
            Self::Queue { from, to } => Some((from, to)),
            Self::ToService => Some((Place::Queue(0), Place::Service)),
            Self::Leaving => Some((Place::Service, Place::Exit)),
            Self::Serving { .. } => Some((Place::Service, Place::Service)),
            _ => None,
        }
    }
    fn waiting(self) -> bool {
        matches!(self, Self::Backlog | Self::Queue { .. } | Self::ToService)
    }
}

fn place_code(p: Place) -> u32 {
    match p {
        Place::Entry => 0,
        Place::Queue(i) => 1 + u32::from(i),
        Place::Service => 257,
        Place::Exit => 258,
    }
}
fn slot(p: Place, s: &TerminalStation) -> Option<usize> {
    match p {
        Place::Entry => None,
        Place::Queue(i) => Some(usize::from(i)),
        Place::Service => Some(s.queue_cells.len()),
        Place::Exit => Some(s.queue_cells.len() + 1),
    }
}
fn cell(p: Place, s: &TerminalStation, build: &Build) -> Option<CellPos> {
    match p {
        Place::Entry => build
            .targets()
            .get(usize::from(s.target))?
            .cells
            .first()
            .copied(),
        Place::Queue(i) => s.queue_cells.get(usize::from(i)).copied(),
        Place::Service => Some(s.service_cell),
        Place::Exit => Some(s.exit_cell),
    }
}
fn heading(a: CellPos, b: CellPos) -> u8 {
    if a == b {
        return ARRIVED;
    }
    let delta = (
        i32::from(b.x) - i32::from(a.x),
        i32::from(b.y) - i32::from(a.y),
    );
    DIRS.iter()
        .position(|&d| d == delta)
        .map_or(UNREACHABLE, |i| i as u8)
}
pub(super) fn can_step(build: &Build, a: CellPos, b: CellPos) -> bool {
    let (Some(from), Some(to)) = (build.grid().cell(a), build.grid().cell(b)) else {
        return false;
    };
    let rules = AccessProfile::Passenger.rules();
    let dx = a.x.abs_diff(b.x);
    let dy = a.y.abs_diff(b.y);
    if a.floor == b.floor {
        if dx > 1 || dy > 1 {
            return false;
        }
        // Same corner rule as FieldSet, but against today's grid, not its old snapshot.
        if dx != 0
            && dy != 0
            && ![
                CellPos {
                    x: a.x,
                    y: b.y,
                    floor: a.floor,
                },
                CellPos {
                    x: b.x,
                    y: a.y,
                    floor: a.floor,
                },
            ]
            .into_iter()
            .all(|at| {
                build
                    .grid()
                    .cell(at)
                    .is_some_and(|c| rules.walkable(c.flags))
            })
        {
            return false;
        }
    } else if dx != 0
        || dy != 0
        || a.floor.abs_diff(b.floor) != 1
        || from.flags & crate::build::grid::STAIRS == 0
        || to.flags & crate::build::grid::STAIRS == 0
    {
        return false;
    }
    rules.step_allowed(from.flags, from.zone, to.flags, to.zone)
}
fn route(group: u8) -> &'static [TerminalStationKind] {
    use TerminalStationKind::*;
    match group {
        GROUP_DEPARTING => &[CheckIn, Security, Gate],
        GROUP_ARRIVING => &[Gate, Exit],
        _ => &[],
    }
}
fn route_index(group: u8, kind: TerminalStationKind) -> Option<usize> {
    route(group).iter().position(|&k| k == kind)
}
pub(super) fn route_valid(group: u8, s: &TerminalStation, stations: &[TerminalStation]) -> bool {
    route_valid_counts(group, s, &route_counts(stations))
}
fn route_counts(stations: &[TerminalStation]) -> [u16; 4] {
    let mut counts = [0; 4];
    for s in stations {
        counts[s.kind as usize] += 1;
    }
    counts
}
fn route_valid_counts(group: u8, s: &TerminalStation, counts: &[u16; 4]) -> bool {
    route_index(group, s.kind).is_some_and(|at| {
        route(group)[at..]
            .iter()
            .all(|kind| counts[*kind as usize] == 1)
    })
}
fn next_target(group: u8, s: &TerminalStation, stations: &[TerminalStation]) -> Option<u8> {
    let at = route_index(group, s.kind)?;
    let kind = route(group).get(at + 1)?;
    let mut matches = stations.iter().filter(|s| s.kind == *kind);
    let next = matches.next()?.target;
    matches.next().is_none().then_some(next)
}
fn terminal(p: &Person, s: &TerminalStation) -> bool {
    route(p.group).last() == Some(&s.kind)
}

#[derive(Default)]
struct Line {
    backlog: VecDeque<usize>,
    admitted: VecDeque<usize>,
    owners: Vec<Option<usize>>,
}
pub(super) struct StationIndex {
    lines: Vec<Line>,
    targets: [Option<usize>; 256],
}
impl Default for StationIndex {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            targets: [None; 256],
        }
    }
}

impl StationIndex {
    fn ensure(&mut self, persons: &[Person], build: &Build) {
        if self.lines.len() == build.stations().len() {
            return;
        }
        self.targets = [None; 256];
        self.lines = build
            .stations()
            .iter()
            .enumerate()
            .map(|(i, s)| {
                self.targets[usize::from(s.target)] = Some(i);
                Line {
                    owners: vec![None; s.queue_cells.len() + 2],
                    ..Line::default()
                }
            })
            .collect();
        let mut ranked = persons
            .iter()
            .enumerate()
            .filter(|(_, p)| p.queue_order != 0)
            .map(|(i, p)| (p.queue_order, i))
            .collect::<Vec<_>>();
        ranked.sort_unstable();
        for (_, i) in ranked {
            let p = &persons[i];
            let Some(si) = self.targets[usize::from(p.target)] else {
                continue;
            };
            let line = &mut self.lines[si];
            let s = &build.stations()[si];
            if p.phase == Phase::Backlog {
                line.backlog.push_back(i);
            } else if let Some((from, to)) = p.phase.edge() {
                line.admitted.push_back(i);
                for place in [from, to] {
                    if let Some(at) = slot(place, s) {
                        line.owners[at] = Some(i);
                    }
                }
            }
        }
    }
}

pub(super) fn advance(
    persons: &mut [Person],
    index: &mut StationIndex,
    telemetry: &mut Vec<StationTelemetry>,
    build: &Build,
    fields: &FieldSet,
) {
    index.ensure(persons, build);
    let counts = route_counts(build.stations());
    let mut max = 0;
    let mut arrivals = Vec::new();
    for (i, p) in persons.iter_mut().enumerate() {
        max = max.max(p.queue_order);
        if p.phase.waiting() {
            p.wait_ticks = p.wait_ticks.saturating_add(1);
        }
        if p.phase == Phase::Approach
            && p.progress == 0.0
            && let Some(si) = index.targets[usize::from(p.target)]
            && cell(Place::Entry, &build.stations()[si], build) == Some(p.pos())
        {
            arrivals.push(i);
        }
    }
    if max.checked_add(persons.len() as u32).is_none() {
        let mut ranked = persons
            .iter()
            .enumerate()
            .filter(|(_, p)| p.queue_order != 0)
            .map(|(i, p)| (p.queue_order, i))
            .collect::<Vec<_>>();
        ranked.sort_unstable();
        for (rank, &(_, i)) in ranked.iter().enumerate() {
            persons[i].queue_order = rank as u32 + 1;
        }
        max = ranked.len() as u32;
    }
    for i in arrivals {
        let p = &mut persons[i];
        max += 1;
        p.queue_order = max;
        p.phase = Phase::Backlog;
        p.state = PASSENGER_WAITING;
        p.heading = ARRIVED;
        p.progress = 0.0;
        index.lines[index.targets[usize::from(p.target)].unwrap()]
            .backlog
            .push_back(i);
    }
    telemetry.clear();
    for (si, s) in build.stations().iter().enumerate() {
        let line = &mut index.lines[si];
        let mut served = 0;
        for &i in &line.admitted {
            let p = &mut persons[i];
            if !route_valid_counts(p.group, s, &counts)
                || (p.phase == Phase::Leaving && terminal(p, s))
            {
                continue;
            }
            match p.phase {
                Phase::Serving { remaining } => {
                    if !walkable_for_passengers(build, p.pos()) {
                        continue;
                    }
                    if remaining > 1 {
                        p.phase = Phase::Serving {
                            remaining: remaining - 1,
                        };
                    } else if terminal(p, s) {
                        p.phase = Phase::Complete;
                        p.state = PASSENGER_COMPLETE;
                        p.queue_order = 0;
                        p.wait_ticks = 0;
                        p.heading = ARRIVED;
                        p.progress = 0.0;
                        line.owners[s.queue_cells.len()] = None;
                        served += 1;
                    } else if next_target(p.group, s, build.stations()).is_some() {
                        // Service ends now. The old service place stays reserved
                        // throughout departure, including a subsequently blocked edge.
                        p.phase = Phase::Leaving;
                        p.heading = heading(s.service_cell, s.exit_cell);
                        line.owners[s.queue_cells.len() + 1] = Some(i);
                        served += 1;
                    }
                }
                Phase::Queue { from, to } if from == to => {
                    let Place::Queue(q) = from else {
                        continue;
                    };
                    let dest = if q == 0 {
                        Place::Service
                    } else {
                        Place::Queue(q - 1)
                    };
                    let at = slot(dest, s).unwrap();
                    let a = cell(from, s, build).unwrap();
                    let b = cell(dest, s, build).unwrap();
                    if line.owners[at].is_none() && can_step(build, a, b) {
                        line.owners[at] = Some(i);
                        p.phase = if q == 0 {
                            Phase::ToService
                        } else {
                            Phase::Queue { from, to: dest }
                        };
                        p.heading = heading(a, b);
                    }
                }
                phase => {
                    let Some((from, to)) = phase.edge() else {
                        continue;
                    };
                    let a = cell(from, s, build).unwrap();
                    let b = cell(to, s, build).unwrap();
                    if !can_step(build, a, b) {
                        continue;
                    }
                    p.progress += WALK_SPEED;
                    if p.progress < 1.0 {
                        continue;
                    }
                    p.x = b.x;
                    p.y = b.y;
                    p.floor = b.floor;
                    p.progress = 0.0;
                    p.heading = ARRIVED;
                    if let Some(at) = slot(from, s) {
                        line.owners[at] = None;
                    }
                    match phase {
                        Phase::ToService => {
                            p.phase = Phase::Serving {
                                remaining: s.service_ticks,
                            }
                        }
                        Phase::Leaving => {
                            line.owners[s.queue_cells.len() + 1] = None;
                            if let Some(target) = next_target(p.group, s, build.stations()) {
                                p.target = target;
                                p.phase = Phase::Approach;
                                p.state = PASSENGER_TRAVELLING;
                                p.queue_order = 0;
                                p.wait_ticks = 0;
                                p.heading = fields.dir(target, p.pos());
                            }
                        }
                        _ => p.phase = Phase::Queue { from: to, to },
                    }
                }
            }
        }
        while line.admitted.front().is_some_and(|&i| {
            persons[i].target != s.target
                || matches!(persons[i].phase, Phase::Approach | Phase::Complete)
        }) {
            line.admitted.pop_front();
        }
        let tail = Place::Queue((s.queue_cells.len() - 1) as u8);
        let tail_slot = slot(tail, s).unwrap();
        if let Some(&i) = line.backlog.front()
            && route_valid_counts(persons[i].group, s, &counts)
            && line.owners[tail_slot].is_none()
            && can_step(
                build,
                cell(Place::Entry, s, build).unwrap(),
                cell(tail, s, build).unwrap(),
            )
        {
            line.backlog.pop_front();
            line.admitted.push_back(i);
            line.owners[tail_slot] = Some(i);
            let p = &mut persons[i];
            p.phase = Phase::Queue {
                from: Place::Entry,
                to: tail,
            };
            p.heading = heading(p.pos(), cell(tail, s, build).unwrap());
        }
        telemetry.push(StationTelemetry {
            target: s.target,
            served,
        });
    }
}

pub(super) fn validate(persons: &[Person], build: &Build) -> Result<(), &'static str> {
    let mut tickets = HashSet::new();
    let mut owners = HashSet::new();
    let mut spatial: Vec<Vec<(u16, u32)>> = vec![Vec::new(); 256];
    let mut backlog_min: [Option<u32>; 256] = [None; 256];
    let counts = route_counts(build.stations());
    let mut targets = [None; 256];
    for s in build.stations() {
        targets[usize::from(s.target)] = Some(s);
    }
    for p in persons {
        if !matches!(p.group, 0 | GROUP_DEPARTING | GROUP_ARRIVING) {
            return Err("personState");
        }
        if p.group == 0 {
            if p.phase != Phase::None || p.state != 0 || p.queue_order != 0 || p.wait_ticks != 0 {
                return Err("personState");
            }
            continue;
        }
        let s = targets[usize::from(p.target)].ok_or("personStation")?;
        if !route_valid_counts(p.group, s, &counts) {
            return Err("personRoute");
        }
        let wait = matches!(
            p.phase,
            Phase::Backlog
                | Phase::Queue { .. }
                | Phase::ToService
                | Phase::Serving { .. }
                | Phase::Leaving
        );
        let expected = if wait {
            PASSENGER_WAITING
        } else if p.phase == Phase::Complete {
            PASSENGER_COMPLETE
        } else {
            PASSENGER_TRAVELLING
        };
        if p.state != expected
            || matches!(p.phase, Phase::None)
            || (wait && (p.queue_order == 0 || !tickets.insert(p.queue_order)))
            || (!wait && (p.queue_order != 0 || p.wait_ticks != 0))
        {
            return Err("personState");
        }
        let stationary = |at: CellPos| p.pos() == at && p.progress == 0.0 && p.heading == ARRIVED;
        match p.phase {
            Phase::Approach => {
                if p.progress >= 1.0
                    || (p.heading > DOWN && p.progress != 0.0)
                    || (p.heading <= DOWN
                        && !approach_next(p).is_some_and(|next| {
                            build.map().contains(next) && floor_index(next.floor).is_some()
                        }))
                {
                    return Err("personPosition");
                }
                if p.heading == ARRIVED && !stationary(cell(Place::Entry, s, build).unwrap()) {
                    return Err("personPosition");
                }
            }
            Phase::Backlog => {
                if !stationary(cell(Place::Entry, s, build).unwrap()) {
                    return Err("personPosition");
                }
                let value = &mut backlog_min[usize::from(s.target)];
                *value = Some(value.map_or(p.queue_order, |old| old.min(p.queue_order)));
            }
            Phase::Complete => {
                if !terminal(p, s) || !stationary(s.service_cell) {
                    return Err("personComplete");
                }
            }
            phase => {
                if phase == Phase::Leaving && terminal(p, s) {
                    return Err("personPhase");
                }
                let (from, to) = phase.edge().ok_or("personState")?;
                let a = cell(from, s, build).ok_or("personSlot")?;
                let b = cell(to, s, build).ok_or("personSlot")?;
                if let Phase::Queue { .. } = phase {
                    let valid = match (from, to) {
                        (Place::Entry, Place::Queue(q)) => {
                            usize::from(q) + 1 == s.queue_cells.len()
                        }
                        (Place::Queue(a), Place::Queue(b)) => a == b || a.checked_sub(1) == Some(b),
                        _ => false,
                    };
                    if !valid {
                        return Err("personSlot");
                    }
                }
                if matches!(phase, Phase::Serving { remaining: 0 }) {
                    return Err("personService");
                }
                if p.pos() != a
                    || !(0.0..1.0).contains(&p.progress)
                    || p.heading != heading(a, b)
                    || (a == b && p.progress != 0.0)
                {
                    return Err("personPosition");
                }
                for place in [from, to] {
                    if place == to && from == to {
                        continue;
                    }
                    if let Some(at) = slot(place, s)
                        && !owners.insert((s.target, at))
                    {
                        return Err("personReservation");
                    }
                }
                // A stationary phase still owns its single place.
                if from == to
                    && let Some(at) = slot(from, s)
                    && !owners.insert((s.target, at))
                {
                    return Err("personReservation");
                }
                let rank = match phase {
                    Phase::Serving { .. } | Phase::Leaving => 0,
                    Phase::ToService => 1,
                    Phase::Queue {
                        from: Place::Queue(q),
                        to,
                    } => 2 + 2 * u16::from(q) - u16::from(to != Place::Queue(q)),
                    _ => 2 * s.capacity + 2,
                };
                spatial[usize::from(s.target)].push((rank, p.queue_order));
            }
        }
    }
    for (target, line) in spatial.iter_mut().enumerate() {
        line.sort_unstable();
        if line.windows(2).any(|w| w[0].1 >= w[1].1)
            || backlog_min[target].is_some_and(|min| line.last().is_some_and(|p| p.1 >= min))
        {
            return Err("personQueueOrder");
        }
    }
    Ok(())
}

pub(super) fn points(
    persons: &[Person],
    build: &Build,
    served: &[StationTelemetry],
) -> Vec<crate::frame::TerminalStationPoint> {
    use crate::frame::TerminalStationPoint;
    let mut by_target = [None; 256];
    let counts = route_counts(build.stations());
    let mut points = build
        .stations()
        .iter()
        .enumerate()
        .map(|(i, s)| {
            by_target[usize::from(s.target)] = Some(i);
            TerminalStationPoint {
                kind: s.kind,
                target: s.target,
                service_cell: s.service_cell,
                capacity: s.capacity,
                service_ticks: s.service_ticks,
                backlog: 0,
                queued: 0,
                serving: 0,
                leaving: 0,
                completed: 0,
                served: 0,
                waiting_ticks: 0,
                max_waiting_ticks: 0,
                blocked: false,
            }
        })
        .collect::<Vec<_>>();
    for t in served {
        if let Some(i) = by_target[usize::from(t.target)] {
            points[i].served = t.served;
        }
    }
    for p in persons {
        if p.group == 0 {
            continue;
        }
        let Some(i) = by_target[usize::from(p.target)] else {
            continue;
        };
        let s = &build.stations()[i];
        let out = &mut points[i];
        out.blocked |= !route_valid_counts(p.group, s, &counts);
        out.blocked |= p.phase == Phase::Leaving && terminal(p, s);
        match p.phase {
            Phase::Backlog => out.backlog += 1,
            Phase::Queue { .. } | Phase::ToService => out.queued += 1,
            Phase::Serving { .. } => out.serving += 1,
            Phase::Leaving => out.leaving += 1,
            Phase::Complete => out.completed += 1,
            _ => {}
        }
        if p.phase.waiting() {
            out.waiting_ticks += u64::from(p.wait_ticks);
            out.max_waiting_ticks = out.max_waiting_ticks.max(p.wait_ticks);
        }
        let edge = match p.phase {
            Phase::Backlog => Some((Place::Entry, Place::Queue((s.queue_cells.len() - 1) as u8))),
            Phase::Queue { from, to } if from == to => {
                if let Place::Queue(q) = from {
                    Some((
                        from,
                        if q == 0 {
                            Place::Service
                        } else {
                            Place::Queue(q - 1)
                        },
                    ))
                } else {
                    None
                }
            }
            _ => p.phase.edge(),
        };
        out.blocked |= if let Some((a, b)) = edge {
            !can_step(
                build,
                cell(a, s, build).unwrap(),
                cell(b, s, build).unwrap(),
            )
        } else {
            p.phase == Phase::Approach && approach_blocked(p, build)
        };
    }
    points
}

fn approach_blocked(p: &Person, build: &Build) -> bool {
    if p.heading == ARRIVED {
        return false;
    }
    approach_next(p).is_none_or(|next| !can_step(build, p.pos(), next))
}

fn approach_next(p: &Person) -> Option<CellPos> {
    let mut next = p.pos();
    if let Some(&(dx, dy)) = DIRS.get(usize::from(p.heading)) {
        let (Some(x), Some(y)) = (
            p.x.checked_add_signed(dx as i16),
            p.y.checked_add_signed(dy as i16),
        ) else {
            return None;
        };
        next.x = x;
        next.y = y;
    } else if p.heading == UP {
        next.floor += 1;
    } else if p.heading == DOWN {
        next.floor -= 1;
    } else {
        return None;
    }
    Some(next)
}
