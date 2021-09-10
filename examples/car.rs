use rand::Rng;
use std::f32::consts::{FRAC_PI_2, PI};

use oge::{Angle, Oge, Script, Vector2};

struct WindowHandler;

impl Script for WindowHandler {
    fn start(_: &mut Oge) -> oge::Result<Self> {
        Ok(Self)
    }

    fn update(&mut self, oge: &mut Oge) {
        if oge.window_has_resized() {
            let top_right = oge.window_dimensions().as_vector2().scale(0.5);
            let bottom_left = top_right.scale(-1.);
            oge.set_window_bounds(oge::Bounds {
                bottom_left,
                top_right,
            });
        }
    }
}

struct Particle {
    position: Vector2,
    velocity: Vector2,
    opacity: f32,
    size: f32,
}

impl Particle {
    fn update(&mut self, delta_time: f32) {
        let drag = (self.velocity.x.powi(2) + self.velocity.y.powi(2)) * 0.005;
        let acceleration = &self.velocity.with_magnitude(-drag);
        self.velocity.add_assign(&acceleration.scale(delta_time));
        self.position.add_assign(&self.velocity.scale(delta_time));
        self.opacity -= 0.5 * delta_time;
    }
}

struct ParticleHandler {
    sprite: oge::Sprite,
    particles: Vec<Particle>,
    max_count: usize,
}

impl ParticleHandler {
    fn new(sprite: oge::Sprite, max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            sprite,
            max_count: max_particles,
        }
    }

    fn spawn(&mut self, rough_position: Vector2, rough_velocity: Vector2, count: u32) {
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            // 0.9 -> 1.1
            let magnitude = rough_velocity.magnitude() * rng.gen::<f32>();
            // ~-23deg -> ~23deg
            let direction = rough_velocity
                .direction()
                .rotate(rng.gen::<f32>() * FRAC_PI_2)
                .rotate(-FRAC_PI_2 * 0.5);
            self.particles.push(Particle {
                position: rough_position.add(&Vector2::new(
                    60. * (rng.gen::<f32>() - 0.5),
                    60. * (rng.gen::<f32>() - 0.5),
                )),
                velocity: Vector2::new_euclidean(direction, magnitude),
                opacity: rng.gen::<f32>() * 0.5 + 0.5,
                size: rng.gen::<f32>() * 3. + 0.5,
            });
            if self.particles.len() >= self.max_count {
                break;
            }
        }
    }

    fn update(&mut self, delta_time: f32) {
        let mut remove_indices = Vec::<usize>::with_capacity(self.particles.capacity());
        for (i, particle) in self.particles.iter_mut().enumerate() {
            particle.update(delta_time);
            if particle.opacity <= 0. {
                remove_indices.push(i);
            }
        }
        for i in remove_indices.into_iter().rev() {
            self.particles.swap_remove(i);
        }
    }

    fn draw_particles(&mut self, oge: &mut Oge) {
        for particle in self.particles.iter() {
            self.sprite.set_position(particle.position);
            self.sprite.set_opacity(particle.opacity);
            self.sprite
                .set_transformation(oge::Matrix2::scale(particle.size));
            oge.draw_once(&self.sprite);
        }
    }
}

struct Car {
    car_sprite: oge::Sprite,
    particle_handler: ParticleHandler,
    is_drifting: bool,
    position: Vector2,
    velocity: Vector2,
    direction: f32,
    torque: f32,
}

impl Script for Car {
    fn start(oge: &mut Oge) -> oge::Result<Self> {
        oge.configure_render_pipeline(oge::RenderPipelineConfiguration {
            anti_aliasing: oge::AntiAliasingMode::Msaa4x,
        });

        let car_sprite = oge.create_sprite(oge::SpriteConfiguration {
            label: Some("Car Sprite"),
            mesh: oge::SpriteMesh::new_rectangle(64., 64.),
            default_texture: oge.create_texture(&oge::TextureConfiguration {
                source: oge::TextureSource::Bytes(include_bytes!("./car.png")),
                filter_mode: oge::FilterMode::Point,
                ..Default::default()
            })?,
            z_index: oge::ZIndex::Specific(1),
            opacity: 1.,
            texture_projection_method: oge::TextureProjectionMethod::ScaleToFit,
        })?;

        let dust_particle_sprite = oge.create_sprite(oge::SpriteConfiguration {
            label: Some("Dust Particle Sprite"),
            mesh: oge::SpriteMesh::new_elipse(4., 4., 8),
            default_texture: oge.create_texture(&oge::TextureConfiguration::color(
                oge::Color::new(0.4, 0.4, 0.4, 1.),
            ))?,
            z_index: oge::ZIndex::Specific(0),
            opacity: 1.,
            texture_projection_method: oge::TextureProjectionMethod::SingleColor,
        })?;
        let particle_handler = ParticleHandler::new(dust_particle_sprite, 4096);

        let velocity = Vector2::ZERO;
        let direction = 0.;
        let torque = 0.;
        let position = Vector2::ZERO;

        Ok(Self {
            car_sprite,
            particle_handler,
            velocity,
            direction,
            torque,
            position,
            is_drifting: false,
        })
    }

    fn update(&mut self, oge: &mut Oge) {
        {
            let delta_torque = 0.5 * oge.delta_time();
            let max_torque = 17.;
            let friction_coefficient = 0.9;
            let get_drag = |torque: f32| torque.powi(2) / max_torque;

            if oge.get_key_down(oge::KeyCode::D) {
                self.torque += delta_torque;
                self.torque -= get_drag(self.torque);
            }
            if oge.get_key_down(oge::KeyCode::A) {
                self.torque -= delta_torque;
                self.torque += get_drag(self.torque);
            }

            self.torque *= friction_coefficient;
            self.direction = self.direction.rotate(self.torque);
        }

        let direction_vector = Vector2::new_euclidean(self.direction, 1.);
        let relative_y_velocity = self.velocity.project(&direction_vector);
        let relative_x_velocity = self.velocity.project(&direction_vector.rotate_90_cw());

        if oge.get_key_down(oge::KeyCode::Space) {
            self.is_drifting = true;
            self.car_sprite.set_opacity(0.7);
        } else {
            self.is_drifting = false;
            self.car_sprite.set_opacity(1.);
        }

        // oge.draw_debug_arrow(
        //     vec![self.position, relative_y_velocity.add(&self.position)],
        //     None,
        // );
        // oge.draw_debug_arrow(
        //     vec![self.position, relative_x_velocity.add(&self.position)],
        //     None,
        // );
        // oge.draw_debug_arrow(
        //     vec![
        //         self.position,
        //         relative_y_velocity
        //             .add(&relative_x_velocity)
        //             .add(&self.position),
        //     ],
        //     Some((Some(oge::Color::BLUE), None, None)),
        // );

        let mut acceleration = if oge.get_key_down(oge::KeyCode::W) {
            direction_vector.scale(1200.)
        } else {
            Vector2::ZERO
        };
        let friction_x = relative_x_velocity
            .scale(relative_x_velocity.magnitude() * 0.2)
            .scale(if self.is_drifting { -0.005 } else { -0.1 });
        let friction = relative_y_velocity.scale(-0.5).add(&friction_x);
        acceleration.add_assign(&friction);

        /* particles */
        {
            let count = (0.000001 * (oge.delta_time() * friction_x.magnitude()).powi(3)) as u32;
            let position = self.position;
            let rough_velocity = friction.scale(-0.08);
            self.particle_handler.spawn(position, rough_velocity, count);
            self.particle_handler.update(oge.delta_time());
        }

        self.velocity
            .add_assign(&acceleration.scale(oge.delta_time()));
        self.position
            .add_assign(&self.velocity.scale(oge.delta_time()));

        {
            let window_bounds = oge.window_bounds();
            let min_y = window_bounds.bottom_left.y - 64.;
            let max_y = window_bounds.top_right.y + 64.;
            let min_x = window_bounds.bottom_left.x - 64.;
            let max_x = window_bounds.top_right.x + 64.;

            if self.position.x > max_x {
                self.position.x = min_x + (self.position.x - max_x);
            } else if self.position.x < min_x {
                self.position.x = max_x + (self.position.x - min_x);
            }
            if self.position.y > max_y {
                self.position.y = min_y + (self.position.y - max_y);
            } else if self.position.y < min_y {
                self.position.y = max_y + (self.position.y - min_y);
            }
        }
    }

    fn render(&mut self, oge: &mut Oge) {
        self.car_sprite.set_position(self.position);
        self.car_sprite
            .set_transformation(oge::Matrix2::rotation(self.direction));
        oge.draw_once(&self.car_sprite);
        self.particle_handler.draw_particles(oge);
    }
}

fn main() -> oge::Result<()> {
    oge::main_loop::start([
        //
        Car::load_script(),
        WindowHandler::load_script(),
    ])
}
