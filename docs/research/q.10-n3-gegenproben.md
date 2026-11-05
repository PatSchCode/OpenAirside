# Q.10-N3 – Reproduzierbare Gegenproben

Stand: 2026-11-05. Produktbasis: `b01ae6c7a382ef9ec596ef0410e20f232f4e6af4`.
Die Tests liefen ausschließlich in einer temporären Quellenkopie; keine Produktkorrektur.

## Aufbau und Testsignal

1. Temporäres Verzeichnis anlegen und mit `git archive b01ae6c Cargo.toml Cargo.lock rust-toolchain.toml core bridge tools/assets content/base` die unveränderte Basis dorthin entpacken. Keine neue Worktree-Verwaltung, Abhängigkeit oder Toolchain.
2. Vor Einfügen der Proben: `cargo test --offline --locked -p openairside-core --lib persons::tests` in der Kopie: fünf bestehende Tests grün (Exit 0).
3. Die beiden folgenden Dateien unter ihren genannten Pfaden in der Kopie anlegen; in `core/src/persons/mod.rs` zusätzlich `#[cfg(test)] mod q10n3_probe;` einbinden. Sonstige Produktdateien unverändert lassen.
4. `cargo test --offline --locked -p openairside-core --no-fail-fast --lib --test q10n3_roundtrip q10n3_ -- --nocapture` in der Kopie ausführen. Die Negativassertionen sind bewusst rot auf dem fehlerhaften Bestand; sie sind kein freigegebener Produktpatch. Positive Kontrollen müssen grün sein.

Ein Compilefehler der ersten Integrationshülle (nicht vorhandener Telemetrie-Accessor) wurde vor Interpretation der Gegenprobe durch den bestehenden öffentlichen Aufruf `served_last_tick` korrigiert. Er ist kein Produktbefund. Offline-Ausführung setzte die vorhandenen Cargo-Downloads voraus; Cargo.lock unverändert.

## Umfang und Grenzen

Die Unit-Proben isolieren Restore, Zählerrekonstruktion und Überlauf. Die Integrationsprobe nutzt die öffentliche Simulation, echten Container mit korrekter Prüfsumme, decode/load_game und weitere Ticks. Zwei unabhängige Live-Simulationen gleicher Konfiguration dienen als positive Determinismuskontrolle. Keine Aussage über beliebige Karten, Plattformen, Leistung oder vollständige Sicherheitsabdeckung.

## Datei: core/src/persons/q10n3_probe.rs

SHA-256: 91f1aa76e5eae26225e3588cfae6540283665c84260cb046a5ba8811b50d6f69.

```rust
//! Isolated diagnostic probes for Q.10-N3; no production repair.
use super::*;

fn person() -> Person {
    Person {
        x: 1,
        y: 1,
        floor: 0,
        heading: ARRIVED,
        target: 3,
        progress: 0.0,
        rng: 1,
        group: GROUP_DEPARTING,
        state: PASSENGER_WAITING,
        queue_order: 0,
        wait_ticks: 0,
    }
}

fn restored(saved: Vec<Person>) -> Persons {
    Persons::restore(
        saved,
        Vec::new(),
        MapSize::new(64, 64).unwrap(),
        8,
        Vec::new(),
    )
    .unwrap()
}

fn stations() -> Vec<TerminalStation> {
    [
        (TerminalStationKind::CheckIn, 3),
        (TerminalStationKind::Security, 5),
        (TerminalStationKind::Gate, 7),
    ]
    .into_iter()
    .map(|(kind, target)| TerminalStation {
        kind,
        target,
        queue_cells: vec![CellPos {
            x: 1,
            y: 1,
            floor: 0,
        }],
        capacity: 1,
        throughput: 1,
    })
    .collect()
}

fn hash(persons: &Persons) -> u64 {
    let mut hasher = StateHasher::new();
    persons.hash_into(&mut hasher);
    hasher.finish()
}

#[test]
fn q10n3_control_valid_waiting_state_loads() {
    let persons = restored(vec![person()]);
    assert_eq!(persons.len(), 1);
    assert_eq!(persons.pool.as_slice()[0].queue_order, 0);
}

#[test]
fn q10n3_control_completed_passenger_keeps_legal_saved_values() {
    let saved = Person {
        target: 7,
        state: PASSENGER_COMPLETE,
        queue_order: 77,
        wait_ticks: 32,
        ..person()
    };
    let loaded = restored(vec![saved]);
    assert_eq!(loaded.pool.as_slice()[0], saved);
}

#[test]
fn q10n3_unknown_group_and_state_must_be_rejected() {
    let map = MapSize::new(64, 64).unwrap();
    let mut invalid_group = person();
    invalid_group.group = 255;
    let mut invalid_state = person();
    invalid_state.state = 255;
    let mut rejected = Vec::new();
    for (label, p) in [("group=255", invalid_group), ("state=255", invalid_state)] {
        let error = Persons::restore(vec![p], Vec::new(), map, 8, Vec::new()).err();
        println!("{label}: restore error = {error:?}");
        rejected.push(error.is_some());
    }
    assert!(
        rejected.into_iter().all(|value| value),
        "unknown passenger codes were accepted"
    );
}

#[test]
fn q10n3_continuation_after_empty_queue_keeps_hash() {
    let stations = stations();
    let mut original = restored(vec![person()]);
    original.enqueue_waiters(&stations);
    original.serve_stations(&stations);
    assert_eq!(original.pool.as_slice()[0].target, 5);
    assert_eq!(original.pool.as_slice()[0].queue_order, 0);
    let bytes = postcard::to_stdvec(&original.saved()).unwrap();
    let mut loaded = restored(postcard::from_bytes(&bytes).unwrap());
    assert_eq!(hash(&original), hash(&loaded), "immediate saved-state hash");
    println!(
        "before next admission: original counter {}, restored counter {}",
        original.next_queue_order, loaded.next_queue_order
    );
    // Both copies reach the same next station; only the invisible counter differs.
    original.pool.items_mut()[0].state = PASSENGER_WAITING;
    loaded.pool.items_mut()[0].state = PASSENGER_WAITING;
    original.enqueue_waiters(&stations);
    loaded.enqueue_waiters(&stations);
    println!(
        "next queue order: original {}, restored {}",
        original.pool.as_slice()[0].queue_order,
        loaded.pool.as_slice()[0].queue_order
    );
    assert_eq!(
        hash(&original),
        hash(&loaded),
        "continuation after admission must remain identical"
    );
}

#[test]
fn q10n3_wraparound_must_not_reverse_fifo() {
    let mut station = stations().remove(1);
    station.queue_cells.push(CellPos {
        x: 2,
        y: 1,
        floor: 0,
    });
    station.capacity = 2;
    let stations = vec![station];
    let earlier = Person {
        target: 5,
        queue_order: u32::MAX,
        ..person()
    };
    let later = Person {
        target: 5,
        ..person()
    };
    let mut persons = restored(vec![earlier, later]);
    persons.enqueue_waiters(&stations);
    persons.serve_stations(&stations);
    println!(
        "after one service: earlier state {}, later state {}",
        persons.pool.as_slice()[0].state,
        persons.pool.as_slice()[1].state
    );
    assert_eq!(
        persons.pool.as_slice()[0].state,
        PASSENGER_COMPLETE,
        "older passenger must be served first"
    );
}

#[test]
fn q10n3_container_unknown_codes_must_be_rejected() {
    use crate::adoption::AdoptionMode;
    use crate::savegame::{Limits, SaveKind, SaveRequest, decode};
    use crate::simulation::{Simulation, SimulationConfig};
    use crate::world::World;
    let map = MapSize::new(64, 64).unwrap();
    let config = SimulationConfig {
        map,
        threads: 1,
        background_threads: 1,
        adoption: AdoptionMode::FixedTick { delay: 1 },
        persons: 0,
        seed: 17,
        start_tick: 0,
    };
    let sim = Simulation::with_world(config, World::small_terminal(map, 1, 17).unwrap()).unwrap();
    let request = SaveRequest {
        name: "Q10N3".into(),
        kind: SaveKind::Manual,
        saved_at: "2026-11-05T14:30:00Z".into(),
        preview: Vec::new(),
    };
    let control = sim.save_snapshot().encode(&request, "", "0.0.0").unwrap();
    let fresh = || Simulation::with_world(config, World::new(map)).unwrap();
    let mut control_target = fresh();
    control_target
        .load_game(decode(&control, &Limits { map_edges: &[64] }).unwrap())
        .unwrap();
    assert_eq!(
        sim.state_hash(),
        control_target.state_hash(),
        "valid control must fully load"
    );
    let mut rejected = Vec::new();
    for field in ["group", "state"] {
        let mut snapshot = sim.save_snapshot();
        match field {
            "group" => snapshot.world.persons[0].group = 255,
            _ => snapshot.world.persons[0].state = 255,
        }
        let bytes = snapshot.encode(&request, "", "0.0.0").unwrap();
        let mut target = fresh();
        let before = target.state_hash();
        let result =
            decode(&bytes, &Limits { map_edges: &[64] }).and_then(|game| target.load_game(game));
        println!(
            "valid container/checksum with {field}=255: load_game accepted = {}",
            result.is_ok()
        );
        rejected.push(result.is_err());
        if result.is_err() {
            assert_eq!(
                before,
                target.state_hash(),
                "rejection must preserve previous world"
            );
        }
    }
    assert!(
        rejected.into_iter().all(|value| value),
        "full container accepted invalid passenger codes"
    );
}
```

## Datei: core/tests/q10n3_roundtrip.rs

SHA-256: 893f07955b13ad10f8033c02b36193ce81ca6e06291b918cfa93dd9231bc3e10.

```rust
//! Q.10-N3: full container roundtrip and subsequent live simulation.
use openairside_core::adoption::AdoptionMode;
use openairside_core::command::Command;
use openairside_core::geometry::MapSize;
use openairside_core::persons::{GROUP_DEPARTING, PASSENGER_TRAVELLING};
use openairside_core::savegame::{Limits, SaveKind, SaveRequest, decode};
use openairside_core::simulation::{Simulation, SimulationConfig};
use openairside_core::time::Speed;
use openairside_core::world::World;

fn config() -> SimulationConfig {
    SimulationConfig {
        map: MapSize::new(64, 64).unwrap(),
        threads: 1,
        background_threads: 1,
        adoption: AdoptionMode::FixedTick { delay: 1 },
        persons: 0,
        seed: 17,
        start_tick: 0,
    }
}

#[test]
fn q10n3_control_two_live_runs_keep_hash() {
    let world_a = World::small_terminal(config().map, 1, 17).unwrap();
    let world_b = World::small_terminal(config().map, 1, 17).unwrap();
    let mut a = Simulation::with_world(config(), world_a).unwrap();
    let mut b = Simulation::with_world(config(), world_b).unwrap();
    a.submit(a.session(), 1, Command::SetSpeed { speed: Speed::X1 });
    b.submit(b.session(), 1, Command::SetSpeed { speed: Speed::X1 });
    for _ in 0..500 {
        a.step();
        b.step();
        assert_eq!(a.state_hash(), b.state_hash());
    }
}

#[test]
fn q10n3_full_save_load_continuation_keeps_hash() {
    let map = config().map;
    let world = World::small_terminal(map, 1, 17).unwrap();
    let mut original = Simulation::with_world(config(), world).unwrap();
    original.submit(
        original.session(),
        1,
        Command::SetSpeed { speed: Speed::X1 },
    );
    let security_target = original
        .world()
        .build()
        .stations()
        .iter()
        .find(|station| {
            station.kind == openairside_core::build::target::TerminalStationKind::Security
        })
        .unwrap()
        .target;
    let mut checkpoint = false;
    for _ in 0..4000 {
        original.step();
        original.world().persons().visit(|person| {
            checkpoint |= person.group == GROUP_DEPARTING
                && person.state == PASSENGER_TRAVELLING
                && original.world().persons().queued_for(security_target) == 0;
        });
        // Check-in must already have served a departure before taking the snapshot.
        let checkin_target = original
            .world()
            .build()
            .stations()
            .iter()
            .find(|station| {
                station.kind == openairside_core::build::target::TerminalStationKind::CheckIn
            })
            .unwrap()
            .target;
        checkpoint &= original.world().persons().served_last_tick(checkin_target) > 0;
        if checkpoint {
            break;
        }
    }
    assert!(
        checkpoint,
        "must reach the empty queue after Check-in service"
    );
    let request = SaveRequest {
        name: "Q10N3 probe".into(),
        kind: SaveKind::Manual,
        saved_at: "2026-11-05T14:30:00Z".into(),
        preview: Vec::new(),
    };
    let bytes = original
        .save_snapshot()
        .encode(&request, "", "0.0.0")
        .unwrap();
    let game = decode(&bytes, &Limits { map_edges: &[64] }).unwrap();
    let mut loaded = Simulation::with_world(config(), World::new(map)).unwrap();
    loaded.load_game(game).unwrap();
    assert_eq!(
        original.state_hash(),
        loaded.state_hash(),
        "hash immediately after complete container load"
    );
    println!(
        "checkpoint tick {}, valid container {} bytes, immediate hash {}",
        original.tick(),
        bytes.len(),
        original.state_hash()
    );
    for step in 1..=4000 {
        original.step();
        loaded.step();
        assert_eq!(
            original.state_hash(),
            loaded.state_hash(),
            "full save/load continuation diverged {step} ticks after checkpoint"
        );
    }
}
```
