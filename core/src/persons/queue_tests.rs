//! ADR-051: legal states, atomic rejection, and FIFO batch boundaries.
use super::*;
use crate::adoption::AdoptionMode;
use crate::savegame::{Limits, SaveKind, SaveRequest, decode};
use crate::simulation::{Simulation, SimulationConfig};
use crate::text::ParamValue;
use crate::world::World;

fn waiting(order: u32) -> Person {
    Person {
        x: 3,
        y: 4,
        floor: 0,
        heading: ARRIVED,
        target: 0,
        progress: 0.0,
        rng: 1,
        group: GROUP_DEPARTING,
        state: PASSENGER_WAITING,
        queue_order: order,
        wait_ticks: 7,
    }
}

fn stations() -> Vec<TerminalStation> {
    vec![TerminalStation {
        kind: TerminalStationKind::CheckIn,
        target: 0,
        queue_cells: (0..3).map(|x| CellPos { x, y: 0, floor: 0 }).collect(),
        capacity: 1,
        throughput: 1,
    }]
}

fn restore(saved: Vec<Person>) -> Result<Persons, &'static str> {
    let persons = Persons::restore(saved, vec![], MapSize::new(64, 64).unwrap(), 1, vec![])?;
    persons.validate_stations(&stations())?;
    Ok(persons)
}

#[test]
fn legal_states_and_early_completion_keep_stored_values() {
    for state in [PASSENGER_WAITING, PASSENGER_COMPLETE] {
        for queue_order in [0, 77, u32::MAX] {
            let person = Person {
                state,
                queue_order,
                ..waiting(0)
            };
            assert_eq!(restore(vec![person]).unwrap().saved(), vec![person]);
        }
    }
    assert!(
        restore(vec![Person {
            state: PASSENGER_TRAVELLING,
            wait_ticks: 0,
            ..waiting(0)
        }])
        .is_ok()
    );
    for kind in [TerminalStationKind::Gate, TerminalStationKind::Exit] {
        let persons = Persons::restore(
            vec![Person {
                group: GROUP_ARRIVING,
                ..waiting(1)
            }],
            vec![],
            MapSize::new(64, 64).unwrap(),
            1,
            vec![],
        )
        .unwrap();
        let mut stations = stations();
        stations[0].kind = kind;
        assert!(persons.validate_stations(&stations).is_ok());
    }
}

#[test]
fn state_reference_order_and_capacity_matrix_rejects_invalid_values() {
    for person in [
        Person {
            group: 255,
            ..waiting(0)
        },
        Person {
            state: 255,
            ..waiting(0)
        },
        Person {
            group: 0,
            ..waiting(0)
        },
        Person {
            group: 0,
            state: 0,
            ..waiting(0)
        },
        Person {
            group: 0,
            state: 0,
            wait_ticks: 0,
            ..waiting(1)
        },
        Person {
            state: PASSENGER_TRAVELLING,
            wait_ticks: 0,
            ..waiting(1)
        },
        Person {
            state: PASSENGER_TRAVELLING,
            ..waiting(0)
        },
    ] {
        assert_eq!(restore(vec![person]).err(), Some("personState"));
    }
    assert_eq!(
        restore(vec![Person {
            group: GROUP_ARRIVING,
            ..waiting(0)
        }])
        .err(),
        Some("personStation")
    );
    let persons = restore(vec![waiting(0)]).unwrap();
    assert_eq!(persons.validate_stations(&[]), Err("personStation"));
    assert_eq!(
        restore(vec![
            waiting(1),
            Person {
                state: PASSENGER_COMPLETE,
                ..waiting(1)
            }
        ])
        .err(),
        Some("personQueueOrder")
    );
    assert_eq!(
        restore((1..=4).map(waiting).collect()).err(),
        Some("personQueueCapacity")
    );
}

#[test]
fn fifo_compacts_whole_batch_and_preserves_complete_orders() {
    for oldest in [u32::MAX, u32::MAX - 1] {
        let mut persons = restore(vec![waiting(oldest), waiting(0), waiting(0)]).unwrap();
        persons.enqueue_waiters(&stations());
        assert_eq!(
            persons
                .saved()
                .iter()
                .map(|p| p.queue_order)
                .collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        persons.serve_stations(&stations());
        assert_eq!(persons.saved()[0].state, PASSENGER_COMPLETE);
        assert_eq!(persons.saved()[1].state, PASSENGER_WAITING);
    }
    let mut persons = restore(vec![
        Person {
            state: PASSENGER_COMPLETE,
            ..waiting(77)
        },
        waiting(0),
    ])
    .unwrap();
    persons.enqueue_waiters(&stations());
    assert_eq!(persons.saved()[1].queue_order, 78);
    let mut loaded = restore(persons.saved()).unwrap();
    persons.enqueue_waiters(&stations());
    loaded.enqueue_waiters(&stations());
    assert_eq!(persons.saved(), loaded.saved());
}

#[test]
fn checksum_valid_containers_reject_atomically() {
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
    let source =
        Simulation::with_world(config, World::small_terminal(map, 1, 17).unwrap()).unwrap();
    let request = SaveRequest {
        name: "validation".into(),
        kind: SaveKind::Manual,
        saved_at: "2026-11-05T14:30:00Z".into(),
        preview: vec![],
    };
    let mut target = Simulation::with_world(config, World::new(map)).unwrap();
    let valid = source
        .save_snapshot()
        .encode(&request, "", "0.0.0")
        .unwrap();
    target
        .load_game(decode(&valid, &Limits { map_edges: &[64] }).unwrap())
        .unwrap();
    assert_eq!(source.state_hash(), target.state_hash());
    for (mutation, check) in [
        (0, "personState"),
        (1, "personState"),
        (2, "personStation"),
        (3, "personQueueOrder"),
        (4, "personQueueCapacity"),
    ] {
        let mut snapshot = source.save_snapshot();
        let first = snapshot.world.persons[0];
        match mutation {
            0 => snapshot.world.persons[0].group = 255,
            1 => snapshot.world.persons[0].state = 255,
            2 => snapshot.world.persons[0].group = GROUP_ARRIVING,
            3 => {
                let p = Person {
                    state: PASSENGER_WAITING,
                    queue_order: 1,
                    ..first
                };
                snapshot.world.persons = vec![p, p];
            }
            _ => {
                let capacity = source.world().build().stations()[0].queue_cells.len();
                snapshot.world.persons = (1..=capacity + 1)
                    .map(|n| Person {
                        state: PASSENGER_WAITING,
                        queue_order: n as u32,
                        ..first
                    })
                    .collect();
            }
        }
        let bytes = snapshot.encode(&request, "", "0.0.0").unwrap();
        let before = (target.session(), target.tick(), target.state_hash());
        let error = target
            .load_game(decode(&bytes, &Limits { map_edges: &[64] }).unwrap())
            .unwrap_err();
        assert_eq!(error.code, "invalidState");
        assert_eq!(
            error.params.get("check"),
            Some(&ParamValue::Text(check.into()))
        );
        assert_eq!(
            before,
            (target.session(), target.tick(), target.state_hash())
        );
    }
}

#[test]
#[ignore = "targeted release-mode load measurement"]
fn admission_load_measurement() {
    for count in [10_000, MAX_PERSONS] {
        for overflow in [false, true] {
            let saved = (0..count)
                .map(|n| Person {
                    state: if n < count / 4 {
                        PASSENGER_COMPLETE
                    } else {
                        PASSENGER_WAITING
                    },
                    queue_order: if n >= count / 2 {
                        0
                    } else {
                        let permuted = (n * 104_729 % count) as u32;
                        if overflow {
                            u32::MAX - permuted
                        } else {
                            permuted + 1
                        }
                    },
                    ..waiting(0)
                })
                .collect::<Vec<_>>();
            let mut ranked = saved
                .iter()
                .enumerate()
                .filter(|(_, p)| p.queue_order != 0)
                .map(|(index, p)| (p.queue_order, index))
                .collect::<Vec<_>>();
            ranked.sort_unstable();
            let mut persons =
                Persons::restore(saved, vec![], MapSize::new(512, 512).unwrap(), 1, vec![])
                    .unwrap();
            let stations = [TerminalStation {
                queue_cells: (0..count)
                    .map(|n| CellPos {
                        x: (n % 512) as u16,
                        y: (n / 512) as u16,
                        floor: 0,
                    })
                    .collect(),
                ..stations().remove(0)
            }];
            persons.validate_stations(&stations).unwrap();
            let start = std::time::Instant::now();
            persons.enqueue_waiters(&stations);
            println!(
                "admission {count}, overflow {overflow}: {:?}",
                start.elapsed()
            );
            let saved = persons.saved();
            let mut prior = 0;
            for (_, index) in ranked {
                assert!(saved[index].queue_order > prior);
                prior = saved[index].queue_order;
            }
            for person in &saved[count / 2..] {
                assert!(person.queue_order > prior);
                prior = person.queue_order;
            }
            persons.validate_stations(&stations).unwrap();
        }
    }
}
