// use oge::{Oge, Script};
// use rand::Rng;
// use std::{
//     f32::consts::PI,
//     time::{Duration, Instant},
// };
// struct WindowHandler;

// impl Script for WindowHandler {
//     fn start(_: &mut Oge) -> Self {
//         Self
//     }

//     fn update(&mut self, oge: &mut Oge) {
//         if oge.window_has_resized() {
//             let window_dimensions = oge.window_dimensions();
//             let (x, y) = (
//                 window_dimensions.width as f32 * 0.5,
//                 window_dimensions.height as f32 * 0.5,
//             );
//             oge.set_window_bounds(oge::Bounds {
//                 bottom_left: oge::Vector2::new(-x, -y),
//                 top_right: oge::Vector2::new(x, y),
//             })
//         }
//     }
// }

// fn draw<T: Fn(f32) -> f32>(func: T) -> Vec<oge::Vector2> {
//     let mut vectors = vec![];
//     for x in -10..10 {
//         let x = (x * 100) as f32;
//         vectors.push(oge::Vector2::new(x, func(x)));
//     }
//     vectors
// }

// macro_rules! vec2 {
//     ($(($x:expr, $y:expr)),* $(,)?) => {
//         vec![
//             $(oge::Vector2::new($x as f32, $y as f32)),*
//         ]
//     };
// }

// struct Line {
//     sprite: oge::Sprite,
//     points: Vec<oge::Vector2>,
//     pen_position: oge::Vector2,
//     last_cursor_position: oge::Vector2,
// }

// impl Line {
//     fn new_sprite(oge: &mut Oge, points: &Vec<oge::Vector2>) -> oge::Sprite {
//         oge::Sprite::new(oge::SpriteConfiguration {
//             label: Some("Shape"),
//             mesh: unsafe {
//                 oge::SpriteMesh::new_line(20., &points, oge::sprite::CurveStyle::PreserveAngles)
//             },
//             texture: oge
//                 .create_texture(&oge::TextureConfiguration {
//                     source: oge::TextureSource::Color(oge::Color::from_rgba8(159, 28, 28, 255)),
//                     filter_mode: oge::FilterMode::Point,
//                     projection_method: oge::TextureProjectionMethod::OneColor,
//                     ..Default::default()
//                 })
//                 .unwrap(),
//         })
//     }
// }

// impl Script for Line {
//     fn start(oge: &mut Oge) -> Self {
//         let points = vec2![(0, 0), (0, 1)];
//         Self {
//             sprite: Line::new_sprite(oge, &points),
//             points,
//             last_cursor_position: oge::Vector2::new(0., 0.),
//             pen_position: oge.get_real_position(&oge.cursor_position()),
//         }
//     }

//     fn update(&mut self, oge: &mut Oge) {
//         let earlier = Instant::now();

//         let cursor_positions = oge.get_real_cursor_positions();
//         let mut count_added = 0;
//         if oge.get_mouse_button_down(oge::MouseButtonCode::Left) {
//             for cursor_position in cursor_positions.into_iter() {
//                 if cursor_position != self.last_cursor_position {
//                     self.pen_position = oge::Vector2::new_euclidean(
//                         cursor_position.relative_direction(&self.pen_position),
//                         self.pen_position.distance_to(&cursor_position) * 0.5,
//                     )
//                     .add(&self.pen_position);
//                     self.points.push(self.pen_position);
//                     self.sprite = Self::new_sprite(oge, &self.points);
//                     self.last_cursor_position = cursor_position;
//                     count_added += 1;
//                 }
//             }
//             {
//                 use std::io::Write;
//                 print!("\r{: >10} vertices", self.points.len() << 1);
//                 std::io::stdout().flush();
//             }
//         }

//         oge.draw_sprites(std::iter::once(&self.sprite));
//     }
// }

// // struct Particle {
// //     position: oge::Vector2,
// //     velocity: oge::Vector2,
// //     kill_date: Instant,
// // }

// // impl Particle {
// //     fn update(&mut self, delta_time: f32) {
// //         // acceleration due to gravity
// //         let mut acceleration = oge::Vector2::new(0.0, -9.8);
// //         // deceleration due to drag
// //         acceleration.sub_assign(
// //             &oge::Vector2::new(self.velocity.x.powi(2), self.velocity.y.powi(2)).scale(0.05),
// //         );
// //         self.velocity.add_assign(&acceleration.scale(delta_time));
// //         self.position.add_assign(&self.velocity);
// //     }
// // }

// // struct ParticleEffects {
// //     particle_sprite: oge::Sprite,
// //     particles: Vec<Particle>,
// // }

// // impl Script for ParticleEffects {
// //     fn start(oge: &mut Oge) -> Self {
// //         Self {
// //             particle_sprite: oge::Sprite::new(oge::SpriteConfiguration {
// //                 label: Some("Particle"),
// //                 mesh: oge::SpriteMesh::new_elipse(3.0, 3.0, 16),
// //                 texture: oge
// //                     .create_texture(&oge::TextureConfiguration {
// //                         source: oge::TextureSource::Color(oge::Color::from_rgba8(159, 28, 28, 255)),
// //                         filter_mode: oge::FilterMode::Point,
// //                         ..Default::default()
// //                     })
// //                     .unwrap(),
// //             }),
// //             particles: vec![],
// //         }
// //     }

// //     fn update(&mut self, oge: &mut Oge) {
// //         if oge.window_has_resized() {
// //             let window_dimensions = oge.window_dimensions();
// //             let (x, y) = (
// //                 window_dimensions.width as f32 * 0.5,
// //                 window_dimensions.height as f32 * 0.5,
// //             );
// //             oge.set_window_bounds(oge::Bounds {
// //                 bottom_left: oge::Vector2::new(-x, -y),
// //                 top_right: oge::Vector2::new(x, y),
// //             });
// //         }

// //         if oge.get_mouse_button_down(oge::MouseButtonCode::Left) {
// //             let mut rng = rand::thread_rng();
// //             for _ in 0..((rng.gen::<u8>() >> 5) + 4) {
// //                 self.particles.push(Particle {
// //                     position: oge.get_real_position(&oge.cursor_position()),
// //                     velocity: {
// //                         oge::Vector2::new_euclidean(
// //                             2. * PI * (rng.gen::<f32>()),
// //                             rng.gen::<f32>() * 2. + 0.5,
// //                         )
// //                     },
// //                     kill_date: Instant::now() + Duration::new(3, 0),
// //                 })
// //             }
// //         }

// //         for particle in self.particles.iter_mut() {
// //             particle.update(oge.delta_time());
// //         }

// //         let now = Instant::now();
// //         for i in (0..self.particles.len()).rev() {
// //             let ref particle = self.particles[i];
// //             if particle.kill_date < now {
// //                 self.particles.remove(i);
// //             }
// //         }
// //     }

// //     fn render(&mut self, oge: &mut Oge) {
// //         let mut rng = rand::thread_rng();
// //         for particle in self.particles.iter() {
// //             self.particle_sprite.set_position(particle.position);
// //             oge.draw_sprites(std::iter::once(&self.particle_sprite));
// //         }
// //     }
// // }

// fn main() -> Result<(), oge::OgeError> {
//     oge::main_loop::start([
//         //
//         Line::load_script(),
//         WindowHandler::load_script(),
//     ])
// }

fn main() {}