use std::{time::{Instant, self}, thread, cmp::max};

use bevy_ecs::system::Resource;

use crate::FRAMES_PER_SECOND;

#[derive(Resource)]
pub struct SimulationClock {
    tick_start: Instant,
    frame_count: u32,
    second_count: u32,
}

impl Default for SimulationClock {
    fn default() -> SimulationClock {
        SimulationClock {
            tick_start: time::Instant::now(),
            frame_count: 0,
            second_count: 0,
        }
    }
}

impl SimulationClock {
    pub fn new() -> SimulationClock {
        SimulationClock::default()
    }

    pub fn sleep_until_next(&mut self) {
        let elapsed = time::Instant::now().duration_since(self.tick_start);
        let target = time::Duration::new(0, 1_000_000_000 / FRAMES_PER_SECOND);
        let sleep_duration = target - elapsed;

        tracing::info!("frame {}.{} processing took {}ms, sleeping {}ms", self.second_count, self.frame_count, elapsed.as_millis(), sleep_duration.as_millis());
        thread::sleep(sleep_duration);
    }

    pub fn get_tick(&self) -> Instant {
        self.tick_start
    }

    pub fn tick(&mut self) {
        self.tick_start = time::Instant::now();
        self.frame_count += 1;
        self.second_count += self.frame_count / FRAMES_PER_SECOND;
        self.frame_count = max(1, self.frame_count % FRAMES_PER_SECOND);
        tracing::info!("starting frame {}.{}", self.second_count, self.frame_count);
    }

    pub fn tick_start(&self) -> Instant {
        return self.tick_start;
    }
}
