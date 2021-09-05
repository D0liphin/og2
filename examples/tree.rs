use oge::{Oge, Script};

struct Tree {
    sprite: oge::Sprite,
}

impl Script for Tree {
    fn start(oge: &mut Oge) -> oge::Result<Self> {
        let tree = Self {
            sprite: oge.create_sprite(oge::SpriteConfiguration {
                label: Some("Tree"),
                mesh: oge::SpriteMesh::new_rectangle(500.0, 500.0),
                texture: &oge::TextureConfiguration {
                    source: oge::TextureSource::Bytes(include_bytes!("./tree.png")),
                    filter_mode: oge::FilterMode::Point,
                    ..Default::default()
                },
                z_index: oge::ZIndex::AboveAll,
            })?,
        };
        Ok(tree)
    }

    fn update(&mut self, oge: &mut Oge) {
        if oge.window_has_resized() {
            let window_dimensions = oge.window_dimensions();
            let (x, y) = (
                window_dimensions.width as f32 * 0.5,
                window_dimensions.height as f32 * 0.5,
            );
            oge.set_window_bounds(oge::Bounds {
                bottom_left: oge::Vector2::new(-x, -y),
                top_right: oge::Vector2::new(x, y),
            })
        }

        oge.draw_sprites([&self.sprite]);
    }
}

fn main() -> oge::Result<()> {
    oge::main_loop::start([Tree::load_script()])
}
