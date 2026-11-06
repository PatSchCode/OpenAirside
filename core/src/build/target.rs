//! Targets of persons (gates, check-in desks, toilets, …) as seed cells for
//! flow fields, and the kind of load they serve.

use serde::{Deserialize, Serialize};

use crate::geometry::CellPos;

use super::access::AccessProfile;

/// Most targets of one world; a target is addressed by one byte.
pub const MAX_TARGETS: usize = 255;

/// A terminal service that owns one passenger target (ADR-048).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum TerminalStationKind {
    CheckIn,
    Security,
    Gate,
    Exit,
}

/// Persistent configuration of a small terminal service. The target is the
/// index into [`super::Build::targets`]; queue cells are reserved for the
/// deterministic FIFO queue processed by the persons module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalStation {
    pub kind: TerminalStationKind,
    pub target: u8,
    pub queue_cells: Vec<CellPos>,
    pub service_cell: CellPos,
    pub exit_cell: CellPos,
    pub capacity: u16,
    pub service_ticks: u16,
}

/// Static geometry only. Later walls may stop a station without making a
/// legitimate save unloadable (ADR-054).
pub(crate) fn validate_stations(
    stations: &[TerminalStation],
    targets: &[Target],
    map: crate::geometry::MapSize,
) -> Result<(), &'static str> {
    let mut cells = std::collections::HashSet::new();
    let mut ids = std::collections::HashSet::new();
    for s in stations {
        let target = targets.get(usize::from(s.target)).ok_or("stations")?;
        if !ids.insert(s.target) {
            return Err("stationTargets");
        }
        if target.cells.len() != 1
            || s.queue_cells.is_empty()
            || s.queue_cells.len() > 256
            || usize::from(s.capacity) != s.queue_cells.len()
            || s.service_ticks == 0
        {
            return Err("stations");
        }
        let entry = target.cells[0];
        for cell in std::iter::once(&entry)
            .chain(s.queue_cells.iter())
            .chain([&s.service_cell, &s.exit_cell])
        {
            if !map.contains(*cell)
                || crate::build::grid::floor_index(cell.floor).is_none()
                || !cells.insert((cell.x, cell.y, cell.floor))
            {
                return Err("stationCells");
            }
        }
        if !neighbours(entry, *s.queue_cells.last().unwrap())
            || !neighbours(s.queue_cells[0], s.service_cell)
            || !neighbours(s.service_cell, s.exit_cell)
            || s.queue_cells.windows(2).any(|p| !neighbours(p[0], p[1]))
        {
            return Err("stationGeometry");
        }
    }
    Ok(())
}

pub(crate) fn neighbours(a: CellPos, b: CellPos) -> bool {
    a.floor == b.floor && u32::from(a.x.abs_diff(b.x)) + u32::from(a.y.abs_diff(b.y)) == 1
}

/// Kind of load that is transported and handled at an airport. Freight is
/// part of the schema for the later cargo extension (ADR-006, FR-046); it has
/// no game mechanics yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase")]
pub enum LoadKind {
    /// Passengers and their baggage.
    Passengers,
    /// Freight (cargo extension after feature parity, FR-048).
    Freight,
}

/// A target: all its cells are reached at distance 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    /// Seed cells; cells that are not walkable are ignored.
    pub cells: Vec<CellPos>,
    /// Kind of load this target serves.
    pub load: LoadKind,
    /// Access of the persons that walk to this target; the flow field is
    /// computed with the rules of this profile (ADR-037).
    pub profile: AccessProfile,
}

impl Target {
    /// Target for passengers on the given cells.
    #[must_use]
    pub fn passengers(cells: Vec<CellPos>) -> Self {
        Self {
            cells,
            load: LoadKind::Passengers,
            profile: AccessProfile::Passenger,
        }
    }

    /// The target for persons with another access profile.
    #[must_use]
    pub fn with_profile(mut self, profile: AccessProfile) -> Self {
        self.profile = profile;
        self
    }
}
