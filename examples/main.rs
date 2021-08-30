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

struct Car {
    sprite: oge::Sprite,
}

impl Script for Car {
    fn start(oge: &mut Oge) -> Self {
        Car {
            sprite: oge::Sprite::new(oge::SpriteConfiguration {
                label: Some("Car"),
                mesh: oge::SpriteMesh::new_rectangle(150.0, 150.0),
                texture: oge.create_texture(&oge::TextureConfiguration {
                    path: IMAGE_DIR.clone().join("car.png"),
                    projection_method: oge::TextureProjectionMethod::ScaleToFit,
                }),
            }),
        }
    }

    fn update(&mut self, oge: &mut Oge) {
        if oge.get_key_down(oge::KeyCode::Right) {
            self.sprite.transform(&oge::Matrix2x2::rotation(1.0 * oge.delta_time()))
        }
        if oge.get_key_down(oge::KeyCode::Left) {
            self.sprite.transform(&oge::Matrix2x2::rotation(-1.0 * oge.delta_time()))
        }
    }

    fn render(&self, oge: &mut Oge) {
        oge.render_sprites(std::iter::once(&self.sprite));
    }

    fn window_resize(&mut self, oge: &mut Oge) {
        let window_dimensions = oge.window_dimensions();
        let (x, y) = (
            window_dimensions.width as f32,
            window_dimensions.height as f32,
        );
        oge.set_window_bounds(oge::Bounds {
            bottom_left: oge::Vector2::new(-x, -y),
            top_right: oge::Vector2::new(x, y),
        })
    } 
}

fn main() {
    oge::main_loop::start([Car::load_script()]);
}
