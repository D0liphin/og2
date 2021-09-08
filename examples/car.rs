use oge::{Oge, Script, Vector2};

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

        Ok(Self {
            car_sprite,
            dust_particle_sprite,
            velocity,
            direction,
            torque,
        })
    }

    fn update(&mut self, oge: &mut Oge) {
        let mut acceleration = Vector2::new_euclidean(self.direction, 1.);
        if oge.get_key_down(oge::KeyCode::W) {
            acceleration.scale_assign(50.);
        }
        
    }

    fn render(&mut self, oge: &mut Oge) {
        self.car_sprite.set_position(self.position);
        oge.draw_once(&self.car_sprite);
    }
}

fn main() -> oge::Result<()> {
    oge::main_loop::start([Car::load_script(), WindowHandler::load_script()])
}
