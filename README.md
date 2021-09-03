<div style="width: 100%; display: flex; justify-content: space-between; margin-bottom: 50px;">
        <h1><span style="font-size: 80px">OG2</span></h1>
        <p>A 2D Game Engine I'm writing for fun.</p>
        <img src="https://imgur.com/KZZi9nS.png" width="250px" height="250px" style="margin-top: -40px"/>
        <p>That's the logo (wow, fancy!)</p> 
</div>

```plaintext
                                                                  Draw
                                                                  Previous
                                                                  Frame
              ┌─ Previous                 ┌─ Previous               │         Update ─┐
              │  Update                   │  Render                 │          Cycle  │
              │  Cycle                    │  Cycle                  │                 │
              │                           │                         │                 │
   ┌──────────┴────────────────────┐ ┌────┴─────────────────────┐ ┌─┴─┐ ┌─────────────┴──
   │                               │ │                          │ │   │ │
   │                               │ │                          │ │   │ │
   │                               │ │                          │ │   │ │
   │                               │ │                          │ │   │ │
══════════════════════════════════════════════════════════════════════════════════════════► t
   │                                                                  │
   └──────────────────────────────────────────────────────────────────┘           ▲
                             oge.delta_time()                                     ┆
                                                                                  ┆
                                   ┆                                              ┆
                                   ┆                                              ┆
                                   └┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┘ called here
```

# Example - 'Particles'

![demonstration - this only looks slow because of the gif](https://imgur.com/KrUqWSP.gif)

## The `oge::Script` trait

This is intended to mimic Unity's `MonoBehaviour` somewhat. Let's implement it on a struct `ParticleEffects`.

```rs
struct ParticleEffects {
    particle_sprite: oge::Sprite,
    particles: Vec<Particle>,
}
```

All scripts need to have a `start(oge: &mut Oge) -> Self` method. This creates an instance of that script.  
Most resources in oge are created using a configuration struct. Here we make an `oge::Sprite` (a mesh with a texture +
some other stuff) using an `oge::SpriteConfiguration`.

```rs
impl Script for ParticleEffects {
    fn start(oge: &mut Oge) -> Self {
        Self {
            particle_sprite: oge::Sprite::new(oge::SpriteConfiguration {
                label: Some("Particle"),
                // Creates an approximation of sphere of radius 4 using 16 vertices
                mesh: oge::SpriteMesh::new_elipse(4.0, 4.0, 16),
                // We create textures using the `Oge` struct.
                texture: oge
                    .create_texture(&oge::TextureConfiguration {
                        // Use a single colour for this texture.
                        source: oge::TextureSource::Color(oge::Color::new(1., 0., 0., 1.)),
                        filter_mode: oge::FilterMode::Point,
                        ..Default::default()
                    })
                    .unwrap(),
            }),
            particles: vec![],
        }
    }
}
```

Let's create our `Particle` struct to handle the particle physics

```rs
struct Particle {
    position: oge::Vector2,
    velocity: oge::Vector2,
    kill_date: Instant,
}

impl Particle {
    fn update(&mut self, delta_time: f32) {
        // acceleration due to gravity
        let mut acceleration = oge::Vector2::new(0.0, -9.8);
        // deceleration due to drag
        acceleration.sub_assign(
            // This approximates the equation for drag equation which is just 
            // 0.5 * k_1 * s^2 * k_2 * k_3 -> k * s^2
            &oge::Vector2::new(self.velocity.x.powi(2), self.velocity.y.powi(2)).scale(0.05),
        );
        self.velocity.add_assign(&acceleration.scale(delta_time));
        self.position.add_assign(&self.velocity);
    }
}
```

Now we need to implement the `update(&mut self, oge: &mut Oge)` function so that our script does something each frame.

```rs
impl Script for ParticleEffects {
    fn start(oge: &mut Oge) -> Self {...}

    fn update(&mut self, oge: &mut Oge) {
        // The coordinate system we used when creating our sphere is not fixed, we need to define it.
        // Each time the window resizes, the viewable region will need to get bigger or smaller so that 
        // our grid remains uniform. 
        // Here we set the viewable region of the window to a region with the 'same' width and height
        // as the window, centered at (0., 0.)
        if oge.window_has_resized() {
            let window_dimensions = oge.window_dimensions();
            let (x, y) = (
                window_dimensions.width as f32 * 0.5,
                window_dimensions.height as f32 * 0.5,
            );
            oge.set_window_bounds(oge::Bounds {
                bottom_left: oge::Vector2::new(-x, -y),
                top_right: oge::Vector2::new(x, y),
            });
        }

        // The mouse could have its state updated multiple times between renders, here, we're limiting
        // ourselves to one click per frame (which is probably fine).
        if oge
            .get_mouse_button_status(oge::MouseButtonCode::Left)
            .pressed_count
            > 0
        {
            let mut rng = rand::thread_rng();
            for _ in 0..((rng.gen::<u8>() >> 3) + 4) {
                self.particles.push(Particle {
                    // calling `oge.cursor_position()` returns the **physical position** of the mouse,
                    // which we then need to convert to our viewable region using `oge.get_real_position(...)`
                    //
                    // This relates to the naming scheme of oge functions. Functions that are nouns are 
                    // pure getters, or very close to pure getters. They shouldn't do any processing of 
                    // data. This means I can use `oge.cursor_position()` anywhere I like and not have
                    // to worry about the impact on performance. `oge.get_real_position()` *does* do some
                    // calculations, so we indicate this with the `get_` prefix.
                    position: oge.get_real_position(oge.cursor_position()),
                    velocity: {
                        // Create a new vector with a given rotation and magnitude
                        oge::Vector2::new_euclidean(
                            PI * (rng.gen::<f32>() - 0.5),
                            rng.gen::<f32>() * 8. + 2.,
                        )
                    },
                    kill_date: Instant::now() + Duration::new(3, 0),
                })
            }
        }

        // update the position of all our particles
        for particle in self.particles.iter_mut() {
            // oge.delta_time() is an example of one of these pure getters. It is effectively instant.
            // This allows us to use it wherever we like and create more readable code.
            particle.update(oge.delta_time());
        }

        // kill particles if they've outlived their kill date
        let now = Instant::now();
        for i in (0..self.particles.len()).rev() {
            let ref particle = self.particles[i];
            if particle.kill_date < now {
                self.particles.remove(i);
            }
        }
    }
}
```

Annoyingly, after all this, our program still does not work! Oge requires us to render sprites explicity. It is convention
to do this in a seperate `render(&mut Oge)` function, but it can be done anywhere.

```rs
impl Script for ParticleEffects {
    fn start(&mut self, oge: &mut Oge) -> Self {...}

    fn update(&mut self, oge: &mut Oge) {...}

    fn render(&mut self, oge: &mut Oge) {
        let mut rng = rand::thread_rng();
        for particle in self.particles.iter() {
            // set the position of our sprite to the position of the particle
            self.particle_sprite.set_position(particle.position);
            // then render it
            oge.draw_sprites(std::iter::once(&self.particle_sprite));
        }
    }
}
```
Finally we need to specify which scripts we want to run. To do this, we need to pass `LoadedScript` objects to our `main_loop::start` 
function. The only way to get these (that should ever be done) is using the auto-implemented `load_script()` function on any 
struct that implementes `oge::Script`.

```rs
fn main() {
    oge::main_loop::start([
        ParticleEffects::load_script()
    ]).unwrap();
}

```
## Tada! It works. \o/

