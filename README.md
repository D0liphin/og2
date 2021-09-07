# Og2

I'm writing this game engine mainly to learn how to structure larger Rust projects and have some fun. Don't use it.  
However, if you are so kind as to read through some of the code, any feedback is much appreciated!  


# Example - 'Tree'

## Description

Displays an image of a tree.

## Screenshot

## Code

```rs
use oge::{Script, Oge};

struct Tree {
    game_object: GameObject,
}

impl Script for Tree {
    fn start(&mut oge: Oge) -> oge::Result<Self> {
        Ok(Self {
            game_object: oge::GameObject::new(oge::GameObjectConfiguration {
                label: Some("Tree"),
                sprite: oge.create_sprite(oge::SpriteConfiguration {
                    label: Some("Tree Sprite"),
                    texture: oge.create_texture(&oge::TextureConfiguration {
                        source: oge::TextureSource::Bytes(include_bytes!("./tree.png")),
                        filter_mode: 
                    })
                })
            })
        })
    }
}

fn main() -> oge::Result<()> {
    oge::main_loop::start([Tree::load_script()])
}
```
