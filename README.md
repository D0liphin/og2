# OG2

```rs
use og2 as oge;
use oge::include_scripts;

struct Player {
    sprite: oge::Sprite,
    velocity: oge::Vector2,
}

impl oge::Script for Player {
    fn start(oge: &Oge) -> Self {
        let sprite = oge::Sprite::new(oge::SpriteConfiguration {
            label: "player_idle",
            mesh: oge::Mesh::new_rectangle(16.0, 16.0),
            texture: oge::Texture::new(oge::TextureConfiguration {
                path: PathBuf::new("./images/player_idle.png"),
            })
        });
        oge.include_sprite_in_render(&sprite);

        Self {
            sprite,
            velocity: oge::Vector2::new(0.0, 0.0), 
        }
    }
}

include_scripts![Player];
```