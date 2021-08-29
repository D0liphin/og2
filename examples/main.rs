use lazy_static::lazy_static;
use oge::{Oge, Script};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

lazy_static! {
    static ref IMAGE_DIR: PathBuf =
        PathBuf::from("/home/oli/Documents/Coding/Rust/Projects/og2/examples/");
}

struct FpsCounter {
    last_update_time_stamp: Instant,
    last_fps_update_time_stamp: Instant,
}

impl Script for FpsCounter {
    fn start(oge: &oge::Oge) -> Self {
        FpsCounter {
            last_update_time_stamp: Instant::now(),
            last_fps_update_time_stamp: Instant::now(),
        }
    }

    fn update(&mut self, oge: &oge::Oge) {
        let now = Instant::now();

        if now.duration_since(self.last_fps_update_time_stamp) > Duration::new(0, 200_000_000) {
            self.last_fps_update_time_stamp = now;
            let delta_time = now.duration_since(self.last_update_time_stamp).as_micros() as f64;
            let updates_per_second = 1_000_000.0 / delta_time;
            print!("{: >10}fps\r", updates_per_second.round());
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }

        self.last_update_time_stamp = now;
    }
}

struct GameCharacter {
    last_update_time_stamp: Instant,
    sprite: oge::Sprite,
}

impl Script for GameCharacter {
    fn start(oge: &Oge) -> Self {
        Self {
            last_update_time_stamp: Instant::now(),
            sprite: oge::Sprite::new(oge::SpriteConfiguration {
                label: Some("Game Character"),
                mesh: oge::SpriteMesh::new_rectangle(0.9, 0.9),
                texture: oge.create_texture(&oge::TextureConfiguration {
                    path: IMAGE_DIR.clone().join("tree.png"),
                    projection_method: oge::TextureProjectionMethod::ScaleToFit,
                }),
            }),
        }
    }

    fn update(&mut self, oge: &Oge) {
        let now = Instant::now();
        let delta_time =
            now.duration_since(self.last_update_time_stamp).as_millis() as f32 / 1000.0;
        self.last_update_time_stamp = now;

        let rotation: f32 = std::f32::consts::PI * delta_time;
        self.sprite.transform(&oge::Matrix3x2 {
            i: oge::Vector2::new(rotation.cos(), -rotation.sin()),
            j: oge::Vector2::new(rotation.sin(), rotation.cos()),
            k: oge::Vector2::new(0.0, 0.0),
        });

        oge.render_sprites(std::iter::once(&self.sprite));
    }
}

fn main() {
    oge::start([FpsCounter::load_script, GameCharacter::load_script]);
}
