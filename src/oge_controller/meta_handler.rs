use crate::*;
use std::time;

pub(crate) struct MetaHandler {
    last_render_time_stamp: time::Instant,
    this_render_time_stamp: time::Instant,
}

impl MetaHandler {
    pub(crate) fn new() -> Self {
        Self {
            last_render_time_stamp: time::Instant::now(),
            this_render_time_stamp: time::Instant::now(),
        }
    }

    pub(crate) fn update(&mut self) {
        self.last_render_time_stamp = self.this_render_time_stamp;
        self.this_render_time_stamp = time::Instant::now();
    }

    pub(crate) fn delta_time(&self) -> f32 {
        self.this_render_time_stamp
            .duration_since(self.last_render_time_stamp)
            .as_micros() as f32
            / 1_000_000.
    }
}
