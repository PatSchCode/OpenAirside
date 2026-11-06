//! Step 3.2 passenger flow: the compact terminal has one deterministic
//! departure and arrival path, and its deliberately slow check-in clears
//! without a permanent deadlock during an hour of game time.

use openairside_core::build::target::TerminalStationKind;
use openairside_core::geometry::MapSize;
use openairside_core::persons::{
    GROUP_ARRIVING, GROUP_DEPARTING, PASSENGER_COMPLETE, PASSENGER_TRAVELLING,
};
use openairside_core::time::TICKS_PER_GAME_HOUR;
use openairside_core::world::World;

fn pool() -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap()
}

#[test]
fn departures_and_arrivals_finish_without_a_permanent_queue() {
    let pool = pool();
    let mut world = World::small_terminal(MapSize::new(64, 64).unwrap(), 80, 17).unwrap();
    world.rebuild_fields_now(&pool);
    for tick in 0..TICKS_PER_GAME_HOUR {
        world.step(&pool, &pool, tick, true);
    }
    let mut departing = 0;
    let mut arriving = 0;
    let mut completed = 0;
    let mut travelling = 0;
    world.persons().visit(|person| {
        departing += u32::from(person.group == GROUP_DEPARTING);
        arriving += u32::from(person.group == GROUP_ARRIVING);
        completed += u32::from(person.state == PASSENGER_COMPLETE);
        travelling += u32::from(person.state == PASSENGER_TRAVELLING);
    });
    assert_eq!((departing, arriving, completed), (40, 40, 80));
    assert_eq!(travelling, 0);
    assert!(!world.persons().has_active());
    assert!(
        world
            .build()
            .stations()
            .iter()
            .all(|station| world.persons().queued_for(station.target) == 0)
    );
}

/// A1: observe the first completion, before later movement can hide an
/// incorrect station transition. The expected point comes from Build,
/// independently of the passenger's success flag.
#[test]
fn first_arrival_completion_is_at_the_exit_and_stays_there() {
    let pool = pool();
    let mut world = World::small_terminal(MapSize::new(64, 64).unwrap(), 2, 17).unwrap();
    world.rebuild_fields_now(&pool);
    let exit = world
        .build()
        .stations()
        .iter()
        .find(|station| station.kind == TerminalStationKind::Exit)
        .unwrap();
    let expected = world.build().targets()[usize::from(exit.target)].cells[0];
    let mut first = None;
    let mut rebuilt_after_completion = false;
    for tick in 0..TICKS_PER_GAME_HOUR {
        if first.is_some() && !rebuilt_after_completion {
            world.rebuild_fields_now(&pool);
            rebuilt_after_completion = true;
        }
        world.step(&pool, &pool, tick, true);
        world.persons().visit(|person| {
            if person.group == GROUP_ARRIVING && person.state == PASSENGER_COMPLETE {
                assert_eq!(
                    (person.x, person.y, person.floor),
                    (
                        f32::from(expected.x) + 0.5,
                        f32::from(expected.y) + 0.5,
                        expected.floor
                    ),
                    "first and later COMPLETE must remain at the exit; tick {tick}"
                );
                if let Some(previous) = first {
                    assert_eq!(person, previous, "COMPLETE moved at tick {tick}");
                }
                first = Some(person);
            }
        });
    }
    assert!(first.is_some(), "arrival never completed");
    assert!(rebuilt_after_completion);
}
