//! Q.10-N3: full container roundtrip and subsequent live simulation.
use openairside_core::adoption::AdoptionMode;
use openairside_core::command::Command;
use openairside_core::geometry::MapSize;
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
    let checkin_target = original
        .world()
        .build()
        .stations()
        .iter()
        .find(|s| s.kind == openairside_core::build::target::TerminalStationKind::CheckIn)
        .unwrap()
        .target;
    let mut checkpoint = false;
    for _ in 0..4000 {
        original.step();
        checkpoint = original
            .world()
            .persons()
            .station_points(original.world().build())
            .iter()
            .any(|s| s.target == checkin_target && s.leaving == 1 && s.served == 1);
        if checkpoint {
            break;
        }
    }
    assert!(
        checkpoint,
        "must capture departure from the real Check-in service cell"
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
