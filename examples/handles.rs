use lazy_static::lazy_static;
use oge::{sprite::Curve, Oge, Script};
use rand::Rng;
use std::{
    f32::consts::PI,
    path::PathBuf,
    time::{Duration, Instant},
};
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
    fn start(_: &mut Oge) -> oge::Result<Self> {
        println!(
            "
 │ FPS COUNTER │
 ├─────────────┤"
        );
        Ok(Self {
            time_of_last_print: Instant::now(),
            total_update_count: 0.,
        })
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

struct WindowHandler;

impl Script for WindowHandler {
    fn start(_: &mut Oge) -> oge::Result<Self> {
        Ok(Self)
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
            })
        }
    }
}

#[derive(Debug)]
struct Handles {
    curve: oge::sprite::Curve,
    joint_sprite: oge::Sprite,
    handling_joint_index: Option<usize>,
}

impl Script for Handles {
    fn start(oge: &mut Oge) -> oge::Result<Self> {
        let red_texture_config =
            oge::TextureConfiguration::color(oge::Color::from_rgba8(119, 15, 15, 255));

        let this = Self {
            curve: oge::sprite::Curve::new(
                oge,
                oge::sprite::CurveConfiguration {
                    label: Some("Curve"),
                    width: 7.,
                    points: vec![oge::Vector2::new(0., 0.), oge::Vector2::new(50., 0.)],
                    texture_configuration: &oge::TextureConfiguration::color(
                        oge::Color::from_rgba8(159, 28, 28, 255),
                    ),
                    style: oge::sprite::CurveStyle::DoubleJointed,
                    z_index: oge::ZIndex::BelowAll,
                    opacity: 1.,
                },
            )
            .unwrap(),
            joint_sprite: oge.create_sprite(oge::SpriteConfiguration {
                label: Some("Joint"),
                mesh: oge::SpriteMesh::new_elipse(12., 12., 16),
                texture: &red_texture_config,
                z_index: oge::ZIndex::AboveAll,
                opacity: 1.,
            })?,
            handling_joint_index: None,
        };
        Ok(this)
    }

    fn update(&mut self, oge: &mut Oge) {
        let cursor_position = oge.get_real_cursor_position();
        let left_mouse_button_status = oge.get_mouse_button_status(oge::MouseButtonCode::Left);

        if self.handling_joint_index.is_none() && left_mouse_button_status.just_pressed() {
            let (joint_index, distance_to_nearest_joint) =
                self.curve.nearest_joint(&cursor_position);
            if distance_to_nearest_joint < 6. {
                self.handling_joint_index = Some(joint_index);
            }
        } else if left_mouse_button_status.just_released() {
            self.handling_joint_index = None;
        }

        if let Some(joint_index) = self.handling_joint_index {
            let joint = self.curve.get_mut(joint_index);
            *joint = cursor_position;
        }

        if oge
            .get_mouse_button_status(oge::MouseButtonCode::Right)
            .just_pressed()
        {
            self.curve.push(cursor_position);
        }

        let space_button_status = oge.get_key_status(oge::KeyCode::Space);
        if space_button_status.just_pressed() {
            oge.configure_render_pipeline(oge::RenderPipelineConfiguration {
                anti_aliasing: oge::AntiAliasingMode::None,
            });
        } else if space_button_status.just_released() {
            oge.configure_render_pipeline(oge::RenderPipelineConfiguration {
                anti_aliasing: oge::AntiAliasingMode::Msaa4x,
            });
        }
    }

    fn render(&mut self, oge: &mut Oge) {
        for point in self.curve.points().iter() {
            self.joint_sprite.set_position(*point);
            oge.draw_sprites([&self.joint_sprite]);
        }
        oge.draw_sprites([self.curve.get_sprite()]);
    }
}

fn main() -> oge::Result<()> {
    oge::main_loop::start([
        WindowHandler::load_script(),
        Handles::load_script(),
        //FpsCounter::load_script(),
    ])
}
