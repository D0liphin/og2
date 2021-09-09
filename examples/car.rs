use std::f32::consts::PI;

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

struct Car {
    car_sprite: oge::Sprite,
    dust_particle_sprite: oge::Sprite,
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

        let velocity = Vector2::ZERO;
        let direction = 0.;
        let torque = 0.;
        let position = Vector2::ZERO;

        Ok(Self {
            car_sprite,
            dust_particle_sprite,
            velocity,
            direction,
            torque,
            position,
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

        let acceleration = {
            let direction_vector = Vector2::new_euclidean(self.direction, 1.);
            let relative_y_velocity = self.velocity.project(&direction_vector);
            let relative_x_velocity = self.velocity.project(&direction_vector.rotate_90_cw());

            oge.draw_debug_arrow(vec![self.position, relative_y_velocity.add(&self.position)], None);
            oge.draw_debug_arrow(vec![self.position, relative_x_velocity.add(&self.position)], None);

            #[allow(unused_mut)]
            let mut acceleration = direction_vector.scale(50.);
            
            acceleration
        };

        self.velocity.add_assign(&acceleration.scale(oge.delta_time()));

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
    }
}

fn main() -> oge::Result<()> {
    oge::main_loop::start([
        //
        Car::load_script(),
        WindowHandler::load_script(),
    ])
}
