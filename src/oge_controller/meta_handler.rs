// use crate::*;
use std::time;

/// Struct for handling meta information about the game. This includes things
/// like time.
pub(crate) struct MetaHandler {
    last_render_time_stamp: time::Instant,
    this_render_time_stamp: time::Instant,
    delta_time: f32,
}

impl MetaHandler {
    pub(crate) fn new() -> Self {
        Self {
            last_render_time_stamp: time::Instant::now(),
            this_render_time_stamp: time::Instant::now(),
            delta_time: 0.0,
        }
    }

    pub(crate) fn update(&mut self) {
        self.last_render_time_stamp = self.this_render_time_stamp;
        self.this_render_time_stamp = time::Instant::now();
        self.delta_time = self.this_render_time_stamp
            .duration_since(self.last_render_time_stamp)
            .as_micros() as f32
            / 1_000_000.
    }

    pub(crate) fn delta_time(&self) -> f32 {
        self.delta_time
    }
}
