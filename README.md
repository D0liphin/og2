# OG2

```rs
use og2 as oge;
use oge::include_scripts;

struct Player {
    sprite: oge::Sprite,
    velocity: oge::Vector2,
}

impl oge::Script for Player {
    fn init(oge: &Oge) -> Self {
        let sprite = oge::Sprite::new(oge::SpriteConfiguration {
            label: "player_idle",
            mesh: oge::SpriteMesh::new_rectangle(0.5, 0.5),
            texture: oge.create_texture(&oge::TextureConfiguration {
                path: PathBuf::new("./images/player_idle.png"),
                projection_method: oge::TextureProjectionMethod::ScaleToFit,
            })
        });

        Self {
            sprite,
            velocity: oge::Vector2::new(0.0, 0.0), 
        }
    }

    fn update(&mut self, oge: &Oge) {
        self.velocity.y += oge.delta_time() * 9.8;
        self.sprite.translate(self.velocity);

        oge.render_sprites(std::iter::once(&self.sprite));
    }
}

fn main() {

}
```