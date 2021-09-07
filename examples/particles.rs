use oge::{Oge, Script};
use rand::Rng;
use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

struct Particle {
    position: oge::Vector2,
    velocity: oge::Vector2,
    opacity: f32,
}

impl Particle {
    fn update(&mut self, delta_time: f32) {
        let mut acceleration = oge::Vector2::new(0.0, -1000.);
        let drag = (self.velocity.x.powi(2) + self.velocity.y.powi(2)) * 0.001;
        acceleration.sub_assign(&self.velocity.with_magnitude(drag));

        self.velocity.add_assign(&acceleration.scale(delta_time));
        self.position.add_assign(&self.velocity.scale(delta_time));
        self.opacity -= 0.2 * delta_time;
    }
}

struct ParticleEffects {
    particle_sprite: oge::Sprite,
    particles: Vec<Particle>,
}

impl Script for ParticleEffects {
    fn start(oge: &mut Oge) -> oge::Result<Self> {
        let script = Self {
            particle_sprite: oge.create_sprite(oge::SpriteConfiguration {
                label: Some("Particle"),
                mesh: oge::SpriteMesh::new_elipse(4., 4., 8),
                default_texture: oge.create_texture(&oge::TextureConfiguration::color(oge::Color::from_rgba8(
                    220, 20, 20, 255,
                )))?,
                ..oge::SpriteConfiguration::default(oge)?
            })?,
            particles: vec![],
        };
        Ok(script)
    }

    fn update(&mut self, oge: &mut Oge) {
        if oge.window_has_resized() {
            let window_dimensions = oge.window_dimensions().as_vector2().scale(-0.5);
            oge.set_window_bounds(oge::Bounds {
                bottom_left: window_dimensions,
                top_right: window_dimensions.scale(-1.),
            });
        }

        if oge
            .get_mouse_button_status(oge::MouseButtonCode::Left)
            .pressed_count
            > 0
        {
            let mut rng = rand::thread_rng();
            for _ in 0..((rng.gen::<u8>() >> 3) + 4) {
                self.particles.push(Particle {
                    position: oge.get_real_position(&oge.cursor_position()),
                    velocity: {
                        oge::Vector2::new_euclidean(
                            PI * (rng.gen::<f32>() - 0.5),
                            rng.gen::<f32>() * 200. + 100.,
                        )
                    },
                    opacity: 1.,
                })
            }
        }

        for particle in self.particles.iter_mut() {
            particle.update(oge.delta_time());
        }

        for i in (0..self.particles.len()).rev() {
            let ref particle = self.particles[i];
            if particle.opacity < 0. {
                self.particles.remove(i);
            }
        }
    }

    fn render(&mut self, oge: &mut Oge) {
        for particle in self.particles.iter() {
            self.particle_sprite.set_position(particle.position);
            self.particle_sprite.set_opacity(particle.opacity);
            oge.draw_once(&self.particle_sprite);
        }
    }
}

fn main() -> oge::Result<()> {
    oge::main_loop::start([ParticleEffects::load_script()])
}
