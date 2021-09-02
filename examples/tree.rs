use oge::{Oge, Script};
use std::time::Instant;

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
            print!("\r │  {: >6} fps │ ", chop_float(self.total_update_count / delta_time, 2));
            std::io::stdout().flush().unwrap_or(());
            self.time_of_last_print = Instant::now();
            self.total_update_count = 0.0;
        }
    }
}

struct Tree {
    sprite: oge::Sprite,
}

impl Script for Tree {
    fn start(oge: &mut Oge) -> Self {
        Self {
            sprite: oge::Sprite::new(oge::SpriteConfiguration {
                label: Some("Tree"),
                mesh: oge::SpriteMesh::new_rectangle(500.0, 500.0),
                texture: oge
                    .create_texture(&oge::TextureConfiguration {
                        source: oge::TextureSource::Bytes(include_bytes!("./tree.png")),
                        filter_mode: oge::FilterMode::Point,
                        ..Default::default()
                    })
                    .unwrap(),
            }),
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
            })
        }

        oge.draw_sprites(std::iter::once(&self.sprite));
    }
}

fn main() -> Result<(), oge::OgeError> {
    oge::main_loop::start([
        //
        Tree::load_script(),
        FpsCounter::load_script(),
    ])
}
