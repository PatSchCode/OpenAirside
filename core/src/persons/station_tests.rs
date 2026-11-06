//! ADR-054/3.2: physical transitions, FIFO before admission and persistence.
use super::station::{Phase, Place, StationIndex};
use super::*;
use crate::adoption::AdoptionMode;
use crate::savegame::{Limits, SaveKind, SaveRequest, decode};
use crate::simulation::{Simulation, SimulationConfig};
use crate::time::Speed;
use crate::world::World;

fn pool() -> ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap()
}
fn config(threads: usize) -> SimulationConfig {
    SimulationConfig {
        map: MapSize::new(64, 64).unwrap(),
        threads,
        background_threads: 1,
        adoption: AdoptionMode::FixedTick { delay: 1 },
        persons: 0,
        seed: 17,
        start_tick: 0,
    }
}
fn passenger(target: u8, phase: Phase, ticket: u32, at: CellPos) -> Person {
    Person {
        x: at.x,
        y: at.y,
        floor: at.floor,
        heading: ARRIVED,
        target,
        progress: 0.0,
        rng: 1,
        group: GROUP_DEPARTING,
        state: if phase == Phase::Approach {
            PASSENGER_TRAVELLING
        } else {
            PASSENGER_WAITING
        },
        queue_order: ticket,
        wait_ticks: 0,
        phase,
    }
}
fn run(persons: &mut [Person], index: &mut StationIndex, build: &Build) {
    let fields = FieldSet::empty(&build.grid().snapshot(), 0);
    station::advance(persons, index, &mut Vec::new(), build, &fields);
    station::validate(persons, build).unwrap();
}
fn request() -> SaveRequest {
    SaveRequest {
        name: "station phases".into(),
        kind: SaveKind::Manual,
        saved_at: "2026-11-06T14:00:00Z".into(),
        preview: vec![],
    }
}

#[test]
fn transitions_are_physical_and_telemetry_counts_every_waiter() {
    let pool = pool();
    let mut world = World::small_terminal(config(1).map, 80, 17).unwrap();
    world.rebuild_fields_now(&pool);
    let mut before = world.persons().saved();
    let mut completed = vec![None; 80];
    let mut visited = std::collections::HashSet::new();
    let mut backlog_seen = false;
    for tick in 0..4000 {
        world.step(&pool, &pool, tick, true);
        let now = world.persons().saved();
        station::validate(&now, world.build()).unwrap();
        let points = world.persons().station_points(world.build());
        for s in world.build().stations() {
            let point = points.iter().find(|p| p.target == s.target).unwrap();
            let waiting = now
                .iter()
                .filter(|p| {
                    p.target == s.target
                        && matches!(
                            p.phase,
                            Phase::Backlog | Phase::Queue { .. } | Phase::ToService
                        )
                })
                .collect::<Vec<_>>();
            assert_eq!(point.backlog + point.queued, waiting.len() as u32);
            assert_eq!(
                point.waiting_ticks,
                waiting.iter().map(|p| u64::from(p.wait_ticks)).sum::<u64>()
            );
            assert_eq!(
                point.max_waiting_ticks,
                waiting.iter().map(|p| p.wait_ticks).max().unwrap_or(0)
            );
            backlog_seen |= point.backlog > 0;
        }
        for (i, p) in now.iter().enumerate() {
            let old = before[i];
            let (a, b) = (old.view(), p.view());
            assert!(
                (a.x - b.x).hypot(a.y - b.y) <= WALK_SPEED + 0.0001,
                "teleport at tick {tick}"
            );
            if p.target != old.target {
                let s = world
                    .build()
                    .stations()
                    .iter()
                    .find(|s| s.target == old.target)
                    .unwrap();
                assert_eq!(old.phase, Phase::Leaving);
                assert_eq!(p.pos(), s.exit_cell);
                assert_eq!(p.phase, Phase::Approach);
            }
            if let Phase::Serving { remaining } = p.phase
                && !matches!(old.phase, Phase::Serving { .. })
            {
                let s = world
                    .build()
                    .stations()
                    .iter()
                    .find(|s| s.target == p.target)
                    .unwrap();
                assert_eq!(p.pos(), s.service_cell);
                assert_eq!(remaining, s.service_ticks);
                visited.insert((p.group, s.kind as u8));
            }
            if p.phase == Phase::Complete {
                let s = world
                    .build()
                    .stations()
                    .iter()
                    .find(|s| s.target == p.target)
                    .unwrap();
                assert_eq!(p.pos(), s.service_cell);
                if let Some(first) = completed[i] {
                    assert_eq!(*p, first);
                } else {
                    completed[i] = Some(*p);
                }
            }
        }
        before = now;
    }
    assert!(backlog_seen);
    assert_eq!(visited.len(), 5);
    assert!(completed.iter().all(Option::is_some));
}

#[test]
fn full_line_preserves_earlier_high_pool_index_ties_and_overflow() {
    let world = World::small_terminal(config(1).map, 0, 17).unwrap();
    let b = world.build();
    let s = &b.stations()[1];
    let entry = b.targets()[usize::from(s.target)].cells[0];
    let mut persons = vec![
        passenger(s.target, Phase::Approach, 0, entry),
        passenger(s.target, Phase::Backlog, 10, entry),
        passenger(
            s.target,
            Phase::Serving { remaining: 30 },
            1,
            s.service_cell,
        ),
        passenger(
            s.target,
            Phase::Queue {
                from: Place::Queue(0),
                to: Place::Queue(0),
            },
            2,
            s.queue_cells[0],
        ),
        passenger(
            s.target,
            Phase::Queue {
                from: Place::Queue(1),
                to: Place::Queue(1),
            },
            3,
            s.queue_cells[1],
        ),
    ];
    let mut index = StationIndex::default();
    station::validate(&persons, b).unwrap();
    run(&mut persons, &mut index, b);
    assert_eq!((persons[1].queue_order, persons[0].queue_order), (10, 11));
    assert_eq!(persons[0].phase, Phase::Backlog);
    for _ in 0..100 {
        run(&mut persons, &mut index, b);
        if persons[1].phase != Phase::Backlog {
            assert_eq!(persons[0].phase, Phase::Backlog);
            break;
        }
    }
    assert_ne!(persons[1].phase, Phase::Backlog);
    let mut ties = vec![passenger(s.target, Phase::Approach, 0, entry); 2];
    run(&mut ties, &mut StationIndex::default(), b);
    assert_eq!((ties[0].queue_order, ties[1].queue_order), (1, 2));
    assert_eq!(ties[1].phase, Phase::Backlog);
    for old in [u32::MAX, u32::MAX - 1] {
        let mut overflow = vec![
            passenger(s.target, Phase::Backlog, old, entry),
            passenger(s.target, Phase::Approach, 0, entry),
        ];
        run(&mut overflow, &mut StationIndex::default(), b);
        assert_eq!((overflow[0].queue_order, overflow[1].queue_order), (1, 2));
    }
}

#[test]
fn upgrade_keeps_the_exact_current_service_remainder() {
    let mut world = World::small_terminal(config(1).map, 0, 17).unwrap();
    let s = world.build().stations()[1].clone();
    let mut persons = vec![passenger(
        s.target,
        Phase::Serving { remaining: 30 },
        1,
        s.service_cell,
    )];
    world
        .build_mut()
        .set_station_service_ticks(s.target, 10)
        .unwrap();
    let mut index = StationIndex::default();
    for elapsed in 1..30 {
        run(&mut persons, &mut index, world.build());
        assert_eq!(
            persons[0].phase,
            Phase::Serving {
                remaining: 30 - elapsed
            }
        );
    }
    run(&mut persons, &mut index, world.build());
    assert_eq!(persons[0].phase, Phase::Leaving);
    assert_eq!(persons[0].pos(), s.service_cell);
    assert_eq!(persons[0].progress, 0.0);
}

#[test]
fn blocked_half_edge_holds_and_reopens_without_teleporting() {
    let mut world = World::small_terminal(config(1).map, 0, 17).unwrap();
    let s = world.build().stations()[1].clone();
    let mut p = passenger(
        s.target,
        Phase::Queue {
            from: Place::Queue(1),
            to: Place::Queue(0),
        },
        1,
        s.queue_cells[1],
    );
    p.heading = 0;
    p.progress = 0.5;
    let mut persons = vec![p];
    let mut index = StationIndex::default();
    let at = s.queue_cells[0];
    let area = crate::geometry::CellRect {
        floor: at.floor,
        x0: at.x,
        y0: at.y,
        x1: at.x + 1,
        y1: at.y + 1,
    };
    world.set_walls(area, true).unwrap();
    for _ in 0..20 {
        run(&mut persons, &mut index, world.build());
    }
    assert_eq!(
        (
            persons[0].pos(),
            persons[0].heading,
            persons[0].progress,
            persons[0].queue_order
        ),
        (p.pos(), p.heading, 0.5, 1)
    );
    assert!(station::points(&persons, world.build(), &[])[1].blocked);
    world.set_walls(area, false).unwrap();
    run(&mut persons, &mut index, world.build());
    assert!((persons[0].progress - 0.6).abs() < 0.0001);
}

#[test]
fn clear_discards_station_indexes_before_empty_steps_and_pool_reuse() {
    let pool = pool();
    let mut world = World::small_terminal(config(1).map, 80, 17).unwrap();
    world.rebuild_fields_now(&pool);
    for tick in 0..100 {
        world.step(&pool, &pool, tick, true);
    }
    assert!(
        world
            .persons()
            .station_points(world.build())
            .iter()
            .any(|p| p.backlog + p.queued > 0)
    );
    world.clear_persons();
    world.step(&pool, &pool, 101, true);
    assert!(world.persons().is_empty());
    assert!(
        world
            .persons()
            .station_points(world.build())
            .iter()
            .all(
                |p| p.backlog + p.queued + p.serving + p.leaving + p.completed == 0
                    && p.served == 0
            )
    );
    assert_eq!(world.spawn_random(3, 17), 3);
    for tick in 102..122 {
        world.step(&pool, &pool, tick, true);
    }
    assert!(
        world
            .persons()
            .saved()
            .iter()
            .all(|p| p.group == 0 && p.phase == Phase::None)
    );
}

#[test]
fn an_approacher_at_a_cell_centre_uses_the_new_route_after_a_build() {
    let pool = pool();
    let mut world = World::small_terminal(config(1).map, 2, 17).unwrap();
    world.rebuild_fields_now(&pool);
    let p = world.persons().saved()[0];
    let (dx, dy) = DIRS[usize::from(p.heading)];
    let at = CellPos {
        x: p.x.checked_add_signed(dx as i16).unwrap(),
        y: p.y.checked_add_signed(dy as i16).unwrap(),
        floor: p.floor,
    };
    let area = crate::geometry::CellRect {
        floor: at.floor,
        x0: at.x,
        y0: at.y,
        x1: at.x + 1,
        y1: at.y + 1,
    };
    world.set_walls(area, true).unwrap();
    let fields = Fields::compute(&crate::wayfinding::Wayfinding::input(world.build()), 0);
    let fresh = fields.main().dir(p.target, p.pos());
    assert!(fresh <= DOWN);
    assert_ne!(fresh, p.heading);
    let moved = step_person(
        p,
        fields.main(),
        world.build().targets().len(),
        world.build(),
    );
    assert_eq!(moved.heading, fresh);
    assert!(moved.progress > 0.0);
}

fn phase_key(p: Phase) -> u8 {
    match p {
        Phase::Approach => 0,
        Phase::Backlog => 1,
        Phase::Queue { .. } => 2,
        Phase::ToService => 3,
        Phase::Serving { .. } => 4,
        Phase::Leaving => 5,
        Phase::Complete => 6,
        Phase::None => 7,
    }
}

#[test]
fn all_phases_roundtrip_and_continue_four_thousand_ticks_across_threads() {
    for wanted in 0..7 {
        let mut source = Simulation::with_world(
            config(1),
            World::small_terminal(config(1).map, 80, 17).unwrap(),
        )
        .unwrap();
        source.submit(
            source.session(),
            1,
            crate::command::Command::SetSpeed { speed: Speed::X1 },
        );
        let mut found = false;
        for _ in 0..4000 {
            source.step();
            found = source.world().persons().saved().iter().any(|p| {
                phase_key(p.phase) == wanted
                    && (wanted != 2 && wanted != 5 || (p.progress > 0.3 && p.progress < 0.7))
            });
            if found {
                break;
            }
        }
        assert!(found, "phase {wanted} not exercised");
        let bytes = source
            .save_snapshot()
            .encode(&request(), "", "0.0.0")
            .unwrap();
        let mut loaded = Simulation::with_world(config(2), World::new(config(2).map)).unwrap();
        loaded
            .load_game(decode(&bytes, &Limits { map_edges: &[64] }).unwrap())
            .unwrap();
        assert_eq!(source.state_hash(), loaded.state_hash());
        for step in 0..4000 {
            source.step();
            loaded.step();
            assert_eq!(
                source.state_hash(),
                loaded.state_hash(),
                "phase {wanted}, step {step}"
            );
        }
    }
}

#[test]
fn checksum_valid_invalid_phases_and_geometry_are_rejected_atomically() {
    let source = Simulation::with_world(
        config(1),
        World::small_terminal(config(1).map, 2, 17).unwrap(),
    )
    .unwrap();
    let mut target = Simulation::with_world(config(1), World::new(config(1).map)).unwrap();
    for mutation in 0..20 {
        let mut snapshot = source.save_snapshot();
        let entry = source.world().build().targets()[0].cells[0];
        match mutation {
            0 => snapshot.world.persons[0].group = 255,
            1 => snapshot.world.persons[0].phase = Phase::None,
            2 => {
                snapshot.world.persons[0] = passenger(
                    0,
                    Phase::Serving { remaining: 0 },
                    1,
                    source.world().build().stations()[0].service_cell,
                )
            }
            3 => {
                let p = passenger(0, Phase::Backlog, 1, entry);
                snapshot.world.persons = vec![p, p];
            }
            4 | 5 => {
                let s = &source.world().build().stations()[if mutation == 4 { 2 } else { 3 }];
                let mut p = passenger(s.target, Phase::Leaving, 1, s.service_cell);
                p.group = if mutation == 4 {
                    GROUP_DEPARTING
                } else {
                    GROUP_ARRIVING
                };
                p.heading = 0;
                snapshot.world.persons[0] = p;
            }
            6 => {
                snapshot.world.persons[0] = passenger(
                    0,
                    Phase::Queue {
                        from: Place::Queue(255),
                        to: Place::Queue(254),
                    },
                    1,
                    entry,
                )
            }
            7 => {
                snapshot.world.persons[0].heading = UNREACHABLE;
                snapshot.world.persons[0].progress = 0.5;
            }
            8 => {
                snapshot.world.persons[0].heading = 0;
                snapshot.world.persons[0].progress = 1.0;
            }
            9 | 10 => {
                let s = &source.world().build().stations()[0];
                let p = passenger(
                    0,
                    Phase::Queue {
                        from: Place::Queue(0),
                        to: Place::Queue(0),
                    },
                    2,
                    s.queue_cells[0],
                );
                let q = passenger(
                    0,
                    Phase::Queue {
                        from: Place::Queue(1),
                        to: Place::Queue(1),
                    },
                    1,
                    s.queue_cells[1],
                );
                snapshot.world.persons = if mutation == 9 {
                    vec![
                        Person {
                            queue_order: 1,
                            ..p
                        },
                        p,
                    ]
                } else {
                    vec![p, q]
                };
            }
            11 => {
                let s = &source.world().build().stations()[2];
                let mut p = passenger(s.target, Phase::Complete, 0, entry);
                p.state = PASSENGER_COMPLETE;
                snapshot.world.persons[0] = p;
            }
            12 | 13 => {
                let s = &source.world().build().stations()[2];
                let mut p = passenger(s.target, Phase::Complete, 0, s.service_cell);
                p.state = PASSENGER_COMPLETE;
                if mutation == 12 {
                    p.queue_order = 1;
                } else {
                    p.wait_ticks = 1;
                }
                snapshot.world.persons[0] = p;
            }
            14 => {
                let s = &source.world().build().stations()[0];
                let mut p = passenger(
                    0,
                    Phase::Queue {
                        from: Place::Queue(1),
                        to: Place::Queue(0),
                    },
                    1,
                    s.queue_cells[1],
                );
                p.heading = UNREACHABLE;
                p.progress = 0.5;
                snapshot.world.persons[0] = p;
            }
            15 => {
                snapshot.world.persons[0].group = 0;
            }
            16 => {
                let p = &mut snapshot.world.persons[0];
                p.x = 63;
                p.heading = 0;
                p.progress = 0.5;
            }
            17 => {
                let p = &mut snapshot.world.persons[0];
                p.floor = 2;
                p.heading = UP;
                p.progress = 0.5;
            }
            18 => {
                let s = &source.world().build().stations()[0];
                let mut p = passenger(
                    0,
                    Phase::Queue {
                        from: Place::Queue(1),
                        to: Place::Queue(0),
                    },
                    1,
                    s.queue_cells[1],
                );
                p.heading = 0;
                p.progress = 1.0;
                snapshot.world.persons[0] = p;
            }
            _ => {
                let p = &mut snapshot.world.persons[0];
                p.group = 0;
                p.phase = Phase::None;
                p.queue_order = 1;
            }
        }
        let bytes = snapshot.encode(&request(), "", "0.0.0").unwrap();
        let before = (target.session(), target.tick(), target.state_hash());
        let error = target
            .load_game(decode(&bytes, &Limits { map_edges: &[64] }).unwrap())
            .unwrap_err();
        assert_eq!(error.code, "invalidState", "mutation {mutation}");
        let expected = match mutation {
            2 => "personService",
            4 | 5 => "personPhase",
            6 => "personSlot",
            7 | 8 | 14 | 16..=18 => "personPosition",
            9 => "personReservation",
            10 => "personQueueOrder",
            11 => "personComplete",
            _ => "personState",
        };
        assert_eq!(
            error.params.get("check"),
            Some(&crate::text::ParamValue::Text(expected.into())),
            "mutation {mutation}"
        );
        assert_eq!(
            before,
            (target.session(), target.tick(), target.state_hash())
        );
    }
}

#[test]
fn saved_upgrade_preserves_a_service_remainder_larger_than_the_new_duration() {
    let mut source = Simulation::with_world(
        config(1),
        World::small_terminal(config(1).map, 80, 17).unwrap(),
    )
    .unwrap();
    source.submit(
        source.session(),
        1,
        crate::command::Command::SetSpeed { speed: Speed::X1 },
    );
    let security = source.world().build().stations()[1].target;
    let mut found = false;
    for _ in 0..4000 {
        source.step();
        found =
            source.world().persons().saved().iter().any(|p| {
                p.target == security && matches!(p.phase, Phase::Serving { remaining: 20.. })
            });
        if found {
            break;
        }
    }
    assert!(found);
    source.submit(
        source.session(),
        2,
        crate::command::Command::SetStationServiceTicks {
            target: security,
            service_ticks: 10,
        },
    );
    source.step();
    assert!(
        source
            .world()
            .persons()
            .saved()
            .iter()
            .any(|p| p.target == security && matches!(p.phase, Phase::Serving { remaining: 11.. }))
    );
    let bytes = source
        .save_snapshot()
        .encode(&request(), "", "0.0.0")
        .unwrap();
    let mut loaded = Simulation::with_world(config(2), World::new(config(2).map)).unwrap();
    loaded
        .load_game(decode(&bytes, &Limits { map_edges: &[64] }).unwrap())
        .unwrap();
    assert_eq!(source.state_hash(), loaded.state_hash());
    for _ in 0..4000 {
        source.step();
        loaded.step();
        assert_eq!(source.state_hash(), loaded.state_hash());
    }
}

#[test]
fn legal_built_blockades_roundtrip_pause_and_resume_every_station_edge() {
    let pool = pool();
    for wanted in [2, 3, 4, 5] {
        let mut world = World::small_terminal(config(1).map, 80, 17).unwrap();
        world.rebuild_fields_now(&pool);
        let mut selected = None;
        for tick in 0..4000 {
            world.step(&pool, &pool, tick, true);
            selected = world.persons().saved().iter().copied().find(|p| {
                phase_key(p.phase) == wanted
                    && (wanted == 4 || (p.progress > 0.3 && p.progress < 0.7))
            });
            if selected.is_some() {
                break;
            }
        }
        let p = selected.unwrap();
        let s = &world.build().stations()[usize::from(p.target)];
        let to = match p.phase {
            Phase::Queue {
                to: Place::Queue(q),
                ..
            } => s.queue_cells[usize::from(q)],
            Phase::ToService | Phase::Serving { .. } => s.service_cell,
            Phase::Leaving => s.exit_cell,
            _ => unreachable!(),
        };
        let area = crate::geometry::CellRect {
            floor: to.floor,
            x0: to.x,
            y0: to.y,
            x1: to.x + 1,
            y1: to.y + 1,
        };
        world.set_walls(area, true).unwrap();
        let mut source = Simulation::with_world(config(1), world).unwrap();
        let before = source.world().persons().saved();
        for _ in 0..20 {
            source.step();
        }
        assert_eq!(
            before,
            source.world().persons().saved(),
            "pause changes phase {wanted}"
        );
        let selected_index = before.iter().position(|record| *record == p).unwrap();
        source.submit(
            source.session(),
            1,
            crate::command::Command::SetSpeed { speed: Speed::X1 },
        );
        for _ in 0..20 {
            source.step();
            let mut held = source.world().persons().saved()[selected_index];
            if matches!(p.phase, Phase::Queue { .. } | Phase::ToService) {
                assert!(held.wait_ticks > p.wait_ticks);
                held.wait_ticks = p.wait_ticks;
            }
            assert_eq!(held, p, "running blockade changed phase {wanted}");
        }
        assert!(
            source
                .world()
                .persons()
                .station_points(source.world().build())
                .iter()
                .any(|s| s.blocked)
        );
        let bytes = source
            .save_snapshot()
            .encode(&request(), "", "0.0.0")
            .unwrap();
        let mut loaded = Simulation::with_world(config(2), World::new(config(2).map)).unwrap();
        loaded
            .load_game(decode(&bytes, &Limits { map_edges: &[64] }).unwrap())
            .unwrap();
        assert_eq!(source.state_hash(), loaded.state_hash());
        for sim in [&mut source, &mut loaded] {
            sim.submit(
                sim.session(),
                2,
                crate::command::Command::Debug {
                    action: crate::command::DebugAction::SetWalls { area, wall: false },
                },
            );
            sim.submit(
                sim.session(),
                3,
                crate::command::Command::SetSpeed { speed: Speed::X1 },
            );
        }
        for _ in 0..4000 {
            source.step();
            loaded.step();
            assert_eq!(source.state_hash(), loaded.state_hash());
        }
        assert!(!source.world().persons().has_active());
    }
}

#[test]
fn security_upgrade_has_a_causal_effect_from_the_same_backlog_snapshot() {
    let mut source = Simulation::with_world(
        config(1),
        World::small_terminal(config(1).map, 80, 17).unwrap(),
    )
    .unwrap();
    source.submit(
        source.session(),
        1,
        crate::command::Command::SetSpeed { speed: Speed::X1 },
    );
    let mut found = false;
    for _ in 0..10800 {
        source.step();
        let points = source
            .world()
            .persons()
            .station_points(source.world().build());
        found = points[3].completed == 40 && points[1].backlog + points[1].queued >= 8;
        if found {
            break;
        }
    }
    assert!(
        found,
        "approved parameters never produce the required observation checkpoint"
    );
    let bytes = source
        .save_snapshot()
        .encode(&request(), "", "0.0.0")
        .unwrap();
    let mut control = Simulation::with_world(config(2), World::new(config(2).map)).unwrap();
    let mut changed = Simulation::with_world(config(2), World::new(config(2).map)).unwrap();
    for sim in [&mut control, &mut changed] {
        sim.load_game(decode(&bytes, &Limits { map_edges: &[64] }).unwrap())
            .unwrap();
    }
    assert_eq!(control.state_hash(), changed.state_hash());
    let security = control.world().build().stations()[1].target;
    let gate = control.world().build().stations()[2].target;
    changed.submit(
        changed.session(),
        1,
        crate::command::Command::SetStationServiceTicks {
            target: security,
            service_ticks: 10,
        },
    );
    let mut areas = [0u64; 2];
    let mut clear = [None; 2];
    let mut last = [(0, 0); 2];
    for tick in 1..=2000 {
        for (i, sim) in [&mut control, &mut changed].into_iter().enumerate() {
            sim.step();
            let points = sim.world().persons().station_points(sim.world().build());
            let waiting = points[1].backlog + points[1].queued;
            areas[i] += u64::from(waiting);
            let saved = sim.world().persons().saved();
            let remaining = saved
                .iter()
                .any(|p| p.group == GROUP_DEPARTING && p.target != gate);
            if !remaining && clear[i].is_none() {
                clear[i] = Some(tick);
            }
            last[i] = (
                waiting,
                saved
                    .iter()
                    .filter(|p| p.group == GROUP_DEPARTING && p.phase == Phase::Complete)
                    .count(),
            );
        }
    }
    println!(
        "checkpoint {}, waiting integrals {:?}, full clearance {:?}, end {:?}",
        source.tick(),
        areas,
        clear,
        last
    );
    assert!(areas[1] < areas[0]);
    assert!(clear[1].unwrap() < clear[0].unwrap());
    assert!(last[1].0 <= last[0].0 && last[1].1 >= last[0].1);
}

/// Construction and restart both reject ambiguous routes before adoption.
#[test]
fn constructed_worlds_with_ambiguous_remaining_routes_are_not_adopted() {
    let invalid = || {
        let mut world = World::small_terminal(config(1).map, 2, 17).unwrap();
        let pos = |x| CellPos { x, y: 0, floor: 0 };
        world
            .build_mut()
            .add_station(
                TerminalStationKind::Gate,
                pos(0),
                vec![pos(1)],
                pos(2),
                pos(3),
                10,
            )
            .unwrap();
        world
    };
    assert!(matches!(
        Simulation::with_world(config(1), invalid()),
        Err(crate::simulation::SimulationError::InvalidState(
            "personRoute"
        ))
    ));
    let mut target = Simulation::with_world(config(1), World::new(config(1).map)).unwrap();
    let before = (target.session(), target.tick(), target.state_hash());
    assert_eq!(
        target.restart_with_world(invalid()).unwrap_err().code,
        "invalidState"
    );
    assert_eq!(
        before,
        (target.session(), target.tick(), target.state_hash())
    );
    assert!(target.take_events(100).iter().any(|r| matches!(&r.event,
        crate::event::Event::Notice{code,..} if code=="invalidState")));
}

/// Replaces the old manual admission measurement. Local Release acceptance
/// (3.2/C4), no CI gate: macOS/Linux/Windows, two tick threads, four stations,
/// 100 warm-up and 2,000 measured full Simulation::step calls; nearest-rank
/// p99. Fixture has permuted tickets, backlog, physical queue and immutable
/// complete records. Failure exits through the Rust harness with 101.
#[test]
#[ignore = "explicit local Release load measurement (3.2/C4)"]
fn full_station_load_acceptance() -> Result<(), &'static str> {
    use std::time::Instant;
    if cfg!(debug_assertions) {
        return Err("run this acceptance with --release");
    }
    for count in [10_000usize, MAX_PERSONS] {
        for overflow in [false, true] {
            let mut world = World::small_terminal(config(2).map, 0, 17).unwrap();
            for target in 0..4 {
                world
                    .build_mut()
                    .set_station_service_ticks(target, 65535)
                    .unwrap();
            }
            let source = Simulation::with_world(config(2), world).unwrap();
            let build = source.world().build();
            let mut records = (0..count)
                .map(|n| {
                    let complete = n % 2 == 0;
                    let target = if complete {
                        2 + (n / 2 % 2) as u8
                    } else {
                        (n / 2 % 4) as u8
                    };
                    let s = &build.stations()[usize::from(target)];
                    let entry = build.targets()[usize::from(target)].cells[0];
                    let mut p = passenger(
                        target,
                        if complete {
                            Phase::Complete
                        } else {
                            Phase::Backlog
                        },
                        if complete {
                            0
                        } else {
                            100 + ((n / 2 * 104729) % (count / 2)) as u32
                        },
                        if complete { s.service_cell } else { entry },
                    );
                    p.group = if s.kind == TerminalStationKind::Exit {
                        GROUP_ARRIVING
                    } else {
                        GROUP_DEPARTING
                    };
                    if complete {
                        p.state = PASSENGER_COMPLETE;
                    }
                    p
                })
                .collect::<Vec<_>>();
            for target in 0..4usize {
                let s = &build.stations()[target];
                let group = if target == 3 {
                    GROUP_ARRIVING
                } else {
                    GROUP_DEPARTING
                };
                records[target] = passenger(
                    target as u8,
                    Phase::Serving { remaining: 65535 },
                    target as u32 + 1,
                    s.service_cell,
                );
                records[target].group = group;
                records[target + 4] = passenger(
                    target as u8,
                    Phase::Queue {
                        from: Place::Queue(0),
                        to: Place::Queue(0),
                    },
                    target as u32 + 5,
                    s.queue_cells[0],
                );
                records[target + 4].group = group;
            }
            if overflow {
                for p in &mut records {
                    if p.queue_order != 0 {
                        p.queue_order += u32::MAX - (count as u32 / 2 + 100);
                    }
                }
            }
            station::validate(&records, build).unwrap();
            let mut snapshot = source.save_snapshot();
            snapshot.world.persons = records;
            let bytes = snapshot.encode(&request(), "", "0.0.0").unwrap();
            let mut sim = Simulation::with_world(config(2), World::new(config(2).map)).unwrap();
            sim.load_game(decode(&bytes, &Limits { map_edges: &[64] }).unwrap())
                .unwrap();
            sim.submit(
                sim.session(),
                1,
                crate::command::Command::SetSpeed { speed: Speed::X1 },
            );
            if overflow {
                let at = Instant::now();
                sim.step();
                let ms = at.elapsed().as_secs_f64() * 1000.0;
                println!(
                    "station load {count}, overflow including index rebuild and full step: {ms:.3} ms"
                );
                assert!(ms <= 100.0, "overflow budget");
                assert_eq!(sim.tick(), 1, "overflow measured no advancing tick");
                assert!(
                    sim.world()
                        .persons()
                        .saved()
                        .iter()
                        .all(|p| p.queue_order <= count as u32),
                    "fixture did not actually force ticket compaction"
                );
            } else {
                for _ in 0..100 {
                    sim.step();
                }
                let mut values = Vec::with_capacity(2000);
                for _ in 0..2000 {
                    let at = Instant::now();
                    sim.step();
                    values.push(at.elapsed().as_secs_f64() * 1000.0);
                }
                let mean = values.iter().sum::<f64>() / values.len() as f64;
                values.sort_by(f64::total_cmp);
                let p99 = values[1979];
                println!(
                    "station load {count}, warmup100, measured2000, mean {mean:.3} ms, nearest-rank p99 {p99:.3} ms"
                );
                let factor = if count == 10_000 { 1.0 } else { 20.0 };
                assert!(mean <= factor && p99 <= factor * 4.0, "normal-path budget");
                assert_eq!(sim.tick(), 2100, "measured steps must really advance");
            }
        }
    }
    Ok(())
}
