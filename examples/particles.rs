use lazy_static::lazy_static;
use oge::{Oge, Script};
use rand::Rng;
use std::{
    f32::consts::PI,
    path::PathBuf,
    time::{Duration, Instant},
};

lazy_static! {
    static ref IMAGE_DIR: PathBuf =
        PathBuf::from("/home/oli/Documents/Coding/Rust/Projects/og2/examples/");
}

struct FpsCounter {
    time_of_last_print: Instant,
    total_update_count: f32,
}

fn chop_float(float: f32, dp: u32) -> String {
    let dp = dp as i32;
    let float_string = format!("{}", float);
    let mut chopped_string = String::new();
    let mut dp_count = -1;
    for c in float_string.chars() {
        if dp_count > -1 {
            dp_count += 1;
        }
        if c == '.' {
            dp_count = 0;
        }
        chopped_string.push(c);
        if dp_count == dp {
            break;
        }
    }
    chopped_string
}

impl Script for FpsCounter {
    fn start(_: &mut Oge) -> Self {
        println!(
            "
 │ FPS COUNTER │
 ├─────────────┤"
        );
        Self {
            time_of_last_print: Instant::now(),
            total_update_count: 0.,
        }
    }

    fn update(&mut self, _: &mut Oge) {
        self.total_update_count += 1.;
        let delta_time = Instant::now()
            .duration_since(self.time_of_last_print)
            .as_micros() as f32
            / 1_000_000.;
        if delta_time > 0.5 {
            use std::io::Write;
            print!(
                "\r │  {: >6} fps │ ",
                chop_float(self.total_update_count / delta_time, 2)
            );
            std::io::stdout().flush().unwrap_or(());
            self.time_of_last_print = Instant::now();
            self.total_update_count = 0.0;
        }
    }
}

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
            &oge::Vector2::new(self.velocity.x.powi(2), self.velocity.y.powi(2)).scale(0.05),
        );
        self.velocity.add_assign(&acceleration.scale(delta_time));
        self.position.add_assign(&self.velocity);
    }
}

struct ParticleEffects {
    particle_sprite: oge::Sprite,
    particles: Vec<Particle>,
}

impl Script for ParticleEffects {
    fn start(oge: &mut Oge) -> Self {
        Self {
            particle_sprite: oge::Sprite::new(oge::SpriteConfiguration {
                label: Some("Particle"),
                mesh: oge::SpriteMesh::new_elipse(4.0, 4.0, 16),
                texture: oge
                    .create_texture(&oge::TextureConfiguration {
                        source: oge::TextureSource::Color(oge::Color::new(1., 0., 0., 1.)),
                        filter_mode: oge::FilterMode::Point,
                        ..Default::default()
                    })
                    .unwrap(),
            }),
            particles: vec![],
        }
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
                            rng.gen::<f32>() * 8. + 2.,
                        )
                    },
                    kill_date: Instant::now() + Duration::new(3, 0),
                })
            }
        }

        for particle in self.particles.iter_mut() {
            particle.update(oge.delta_time());
        }

        let now = Instant::now();
        for i in (0..self.particles.len()).rev() {
            let ref particle = self.particles[i];
            if particle.kill_date < now {
                self.particles.remove(i);
            }
        }
    }

    fn render(&mut self, oge: &mut Oge) {
        let mut rng = rand::thread_rng();
        for particle in self.particles.iter() {
            self.particle_sprite.set_position(particle.position);
            oge.draw_sprites(std::iter::once(&self.particle_sprite));
        }
    }
}

fn main() {
    oge::main_loop::start([ParticleEffects::load_script(), FpsCounter::load_script()]).unwrap();
}
