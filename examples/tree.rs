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
                default_texture: oge.create_texture(&oge::TextureConfiguration {
                    source: oge::TextureSource::Bytes(include_bytes!("./tree.png")),
                    filter_mode: oge::FilterMode::Point,
                    ..Default::default()
                })?,
                z_index: oge::ZIndex::AboveAll,
                opacity: 1.,
                texture_projection_method: oge::TextureProjectionMethod::ScaleToFit,
            })?,
        };
        Ok(tree)
    }

    fn update(&mut self, oge: &mut Oge) {
        if oge.window_has_resized() {
            let (x, y) = (
                oge.window_dimensions().width as f32 * 0.5,
                oge.window_dimensions().height as f32 * 0.5,
            );
            oge.set_window_bounds(oge::Bounds {
                bottom_left: oge::Vector2::new(-x, -y),
                top_right: oge::Vector2::new(x, y),
            })
        }
    }

    fn render(&mut self, oge: &mut Oge) {
        oge.draw_once(&self.sprite);
    }
}

fn main() -> oge::Result<()> {
    oge::main_loop::start([Tree::load_script()])
}
