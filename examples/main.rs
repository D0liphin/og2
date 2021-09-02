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

#[macro_use]
macro_rules! printfl {
    ($($arg:tt)*) => {
        {
            use std::io::Write;
            print!($($arg)*);
            std::io::stdout().flush().unwrap_or(());
        }
    };
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

struct FpsCounter {
    time_of_last_print: Instant,
    total_update_count: f32,
}

impl Script for FpsCounter {
    fn start(_: &mut Oge) -> Self {
        Self {
            time_of_last_print: Instant::now(),
            total_update_count: 0.,
        }
    }

    fn update(&mut self, oge: &mut Oge) {
        self.total_update_count += 1.;
        let delta_time = Instant::now()
            .duration_since(self.time_of_last_print)
            .as_micros() as f32
            / 1_000_000.;
        if delta_time > 0.5 {
            printfl!("{: >12}fps\r", self.total_update_count / delta_time);
            self.time_of_last_print = Instant::now();
            self.total_update_count = 0.0;
        }
    }
}

struct Car {
    sprite: oge::Sprite,
    position: oge::Vector2,
    velocity: oge::Vector2,
    speed: f32,
    torque: f32,
    angle: f32,
}

impl Script for Car {
    fn start(oge: &mut Oge) -> Self {
        Car {
            sprite: oge::Sprite::new(oge::SpriteConfiguration {
                label: Some("Car"),
                mesh: oge::SpriteMesh::new_rectangle(64.0, 64.0),
                texture: oge
                    .create_texture(&oge::TextureConfiguration {
                        source: oge::TextureSource::Bytes(include_bytes!("./car.png")),
                        filter_mode: oge::FilterMode::Point,
                        ..Default::default()
                    })
                    .unwrap(),
            }),
            position: oge::Vector2::new(0.0, 0.0),
            velocity: oge::Vector2::new(0.0, 0.0),
            speed: 0.0,
            angle: 0.0,
            torque: 0.0,
        }
    }

    fn update(&mut self, oge: &mut Oge) {
        let window_dimensions = oge.window_dimensions();
        let (x, y) = (
            window_dimensions.width as f32 * 0.5,
            window_dimensions.height as f32 * 0.5,
        );
        oge.set_window_bounds(oge::Bounds {
            bottom_left: oge::Vector2::new(-x, -y),
            top_right: oge::Vector2::new(x, y),
        });

        let cursor_position = oge.get_real_position(oge.cursor_position());
        let direction_vector = oge::Vector2::new_euclidean(self.angle, 1.0);
        let car_cursor_angle =
            oge::Vector2::angle_between(&direction_vector, &(cursor_position.sub(&self.position)));

        if self.speed > 5.0 {
            if car_cursor_angle.is_sign_positive() {
                self.torque += oge.delta_time() * 0.2 * car_cursor_angle.powi(2);
                if self.torque > 0.15 {
                    self.torque = 0.15;
                }
            } else {
                self.torque -= oge.delta_time() * 0.2 * car_cursor_angle.powi(2);
                if self.torque < -0.15 {
                    self.torque = -0.15;
                }
            }
        }
        self.torque *= oge.delta_time() * -5.0 * self.torque + 0.95;
        self.angle += self.torque
            * if self.speed < 200.0 {
                0.005 * self.speed
            } else {
                1.0
            };

        if oge.get_mouse_button_down(oge::MouseButtonCode::Left) {
            let distance =
                (self.speed + 20.0) * 0.001 * self.position.distance_to(&cursor_position);
            let distance = if distance > 20.0 { 20.0 } else { distance };
            self.speed += distance;

            let max_speed = distance * 10.0;

            if self.speed > max_speed {
                self.speed -= distance;
                if self.speed < max_speed {
                    self.speed = max_speed;
                }
            }
        } else {
            self.speed -= 300.0 * self.torque.abs();
            if self.speed < 0.0 {
                self.speed = 0.0;
            }
        }

        self.velocity.scale_assign(0.95);
        self.velocity
            .add_assign(&direction_vector.scale(self.speed));
        let velocity_magnitude = self.velocity.magnitude();
        if velocity_magnitude > 800.0 {
            self.velocity.scale_assign(800.0 / velocity_magnitude);
        }

        self.position
            .add_assign(&self.velocity.scale(oge.delta_time()));

        self.sprite
            .set_transformation(oge::Matrix2::rotation(self.angle));
        self.sprite.set_position(self.position);
    }

    fn render(&mut self, oge: &mut Oge) {
        oge.draw_sprites(std::iter::once(&self.sprite));
    }
}

struct RateLimiter {
    last_permit: Instant,
    interval: Duration,
}

impl RateLimiter {
    fn new(interval: Duration) -> Self {
        Self {
            last_permit: Instant::now(),
            interval,
        }
    }

    fn rate_limited(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_permit) > self.interval {
            self.last_permit = now;
            false
        } else {
            true
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
    rate_limiter: RateLimiter,
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
            rate_limiter: RateLimiter::new(Duration::new(0, 20_000_000)),
        }
    }

    fn update(&mut self, oge: &mut Oge) {
        if !self.rate_limiter.rate_limited() {
            self.particles.push(Particle {
                position: oge.get_real_position(oge.cursor_position()),
                velocity: {
                    let mut rng = rand::thread_rng();
                    oge::Vector2::new_euclidean(
                        PI * (rng.gen::<f32>() - 0.5),
                        rng.gen::<f32>() * 4. + 2.,
                    )
                },
                kill_date: Instant::now() + Duration::new(3, 0),
            });
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
        for particle in self.particles.iter() {
            self.particle_sprite.set_position(particle.position);
            oge.draw_sprites(std::iter::once(&self.particle_sprite));
        }
    }
}

fn main() {
    oge::main_loop::start([
        Car::load_script(),
        ParticleEffects::load_script(),
        FpsCounter::load_script(),
    ])
    .unwrap();
}
