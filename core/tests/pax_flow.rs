//! Step 3.2 passenger flow: the compact terminal has one deterministic
//! departure and arrival path, and its deliberately slow check-in clears
//! without a permanent deadlock during an hour of game time.

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
