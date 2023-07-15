mod simulation_clock;

use std::thread;

use bevy_ecs::prelude::*;

use simulation_clock::SimulationClock;

const FRAMES_PER_SECOND: u32 = 1;

fn main() {
    // logging
    tracing_subscriber::fmt::init();

    // Initialize and start the ECS
    let mut world = World::new();
    let mut schedule = init_ecs(&mut world);

    thread::scope(|s| {
        // Spawn ECS in background thread, scoped so we can borrow world later
        s.spawn(|| loop {
            web_server::run_leptos();
        });

        s.spawn(|| loop {
            schedule.run(&mut world);
        });
    });

    // start interactive console on main thread here
    // ...
}

/// System to starts a new tick using the simulation clock
fn start_tick(mut sim_clock: ResMut<SimulationClock>) {
    sim_clock.tick();
}

/// System to end the current tick using the simulation clock
fn end_tick(mut sim_clock: ResMut<SimulationClock>) {
    sim_clock.sleep_until_next();
}

/// Initializes the ECS, returning the schedule with all configured systems.
fn init_ecs(world: &mut World) -> Schedule {
    world.init_resource::<SimulationClock>();
    let mut schedule = Schedule::default();
    schedule.add_systems(
        (
            // log start time for tick
            start_tick,

            // ... do stuff

            // sleep until next tick is supposed to start
            end_tick,
        )
        .chain(),
    );
    return schedule;
}