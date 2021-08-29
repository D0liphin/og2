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

struct Tree {
    sprite: oge::Sprite,
}

impl Script for Tree {
    fn start(oge: &mut Oge) -> Self {
        Tree {
            sprite: oge::Sprite::new(oge::SpriteConfiguration {
                label: Some("Tree"),
                mesh: oge::SpriteMesh::new_rectangle(20.0, 20.0),
                texture: oge.create_texture(&oge::TextureConfiguration {
                    path: IMAGE_DIR.clone().join("tree.png"),
                    projection_method: oge::TextureProjectionMethod::ScaleToFit,
                }),
            }),
        }
    }

    fn update(&mut self, oge: &mut Oge) {
        oge.render_sprites(std::iter::once(&self.sprite));
    }

    fn window_resize(&mut self, oge: &mut Oge) {
        let window_dimensions = oge.window_dimensions();
        let (x, y) = (
            window_dimensions.width as f32 * 0.02,
            window_dimensions.height as f32 * 0.02,
        );
        oge.set_window_bounds(oge::Bounds {
            bottom_left: oge::Vector2::new(-x, -y),
            top_right: oge::Vector2::new(x, y),
        })
    }
}

fn main() {
    oge::main_loop::start([Tree::load_script()]);
}
