use lazy_static::lazy_static;
use oge::{Oge, Script};
use rand::Rng;
use std::{
    f32::consts::PI,
    path::PathBuf,
    time::{Duration, Instant},
};

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

fn main() {
    oge::main_loop::start([Car::load_script()]).unwrap();
}
