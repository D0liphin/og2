// use oge::{load_window_handler, Angle, Color, Oge, Script, Sprite, Vector2};
// use std::{
//     f32::consts::{FRAC_PI_2, PI},
//     io::{self, Write},
// };

// macro_rules! draw_debug_line {
//     ($oge:expr, $line:expr, $color:expr) => {{
//         let mut curve = oge::sprite::Curve::new(
//             $oge,
//             oge::sprite::CurveConfiguration {
//                 label: Some("Debug Line"),
//                 points: vec![$line.point_at(-2000.), $line.point_at(2000.)],
//                 width: 3.,
//                 style: oge::CurveStyle::PreserveAngles,
//                 texture_configuration: &oge::TextureConfiguration {
//                     source: oge::TextureSource::Color($color),
//                     projection_method: oge::TextureProjectionMethod::SingleColor,
//                     ..Default::default()
//                 },
//             },
//         )
//         .unwrap();
//         $oge.draw_sprites([curve.get_sprite()]);
//     }};
// }

// #[derive(Debug, Clone, Copy)]
// struct Joint {
//     position: Vector2,
//     velocity: Vector2,
// }

// impl Joint {
//     const ZERO: Self = Joint {
//         position: Vector2::ZERO,
//         velocity: Vector2::ZERO,
//     };
// }

// #[derive(Debug, Clone, Copy)]
// struct Arm {
//     backup: Joint,
//     joint: Joint,
//     length: f32,
//     last_magnitude: f32,
// }

// impl Arm {
//     fn new(joint: Joint, length: f32) -> Self {
//         Self {
//             backup: joint,
//             joint,
//             length,
//             last_magnitude: 0.,
//         }
//     }

//     fn update(&mut self, oge: &mut Oge, pivot: &Joint) {
//         let old_position = self.joint.position;

//         self.backup = self.joint;
//         // draw_debug_line!(oge, oge::Line::connect(&self.joint.position, &pivot.position), oge::Color::BLUE);
//         // draw_debug_line!(oge, pivot_line_of_motion, oge::Color::BLUE);
//         // draw_debug_line!(oge, line_of_motion, oge::Color::BLUE);
//         if pivot.velocity != Vector2::ZERO {
//             let arm_vector = pivot.position.sub(&self.joint.position);
//             let pivot_line_of_motion = oge::Line {
//                 position: pivot.position,
//                 direction: pivot.velocity,
//             };
//             let line_of_motion = oge::Line {
//                 position: self.joint.position,
//                 direction: pivot.velocity.rotate_90_cw(),
//             };
//             let mut goal_position_option =
//                 if oge::Vector2::angle_between(&pivot.velocity, &arm_vector) > 0. {
//                     oge::Line::intersection(&pivot_line_of_motion, &line_of_motion)
//                 } else {
//                     None
//                 };

//             if let Some(goal_position) = goal_position_option {
//                 let square_magnitude =
//                     self.length.powi(2) - goal_position.distance_to(&pivot.position).powi(2);
//                 if square_magnitude < 0. {
//                     goal_position_option = None;
//                 } else {
//                     self.joint.position = self
//                         .joint
//                         .position
//                         .sub(&goal_position)
//                         .with_magnitude({
//                             let magnitude = square_magnitude.sqrt();
//                             println!("{}", magnitude);
//                             magnitude
//                         })
//                         .add(&goal_position);
//                 }
//             }
//             if goal_position_option.is_none() {
//                 self.joint.position = self
//                     .joint
//                     .position
//                     .sub(&pivot.position)
//                     .with_magnitude(self.length - 0.1)
//                     .add(&pivot.position);
//             }

//             self.joint.velocity = self.joint.position.sub(&old_position);

//             pivot.velocity.rotate_90_cw();
//         }

//         if self.joint.position.x.is_nan() {
//             *self = Arm {
//                 joint: self.backup,
//                 ..*self
//             }
//         }
//     }
// }

// struct Pendulum {
//     curve: oge::sprite::Curve,
//     handle: Joint,
//     arms: Vec<Arm>,
//     joint_sprite: (oge::Sprite, oge::Sprite),
// }

// impl Script for Pendulum {
//     fn start(oge: &mut Oge) -> Self {
//         let red_texture_config = oge::TextureConfiguration {
//             source: oge::TextureSource::Color(oge::Color::from_rgba8(220, 20, 20, 255)),
//             projection_method: oge::TextureProjectionMethod::SingleColor,
//             ..Default::default()
//         };
//         Self {
//             curve: oge::sprite::Curve::new(
//                 oge,
//                 oge::sprite::CurveConfiguration {
//                     label: Some("Pendulum"),
//                     width: 8.,
//                     points: [Vector2::ZERO; 2].to_vec(),
//                     style: oge::sprite::CurveStyle::DoubleJointed,
//                     texture_configuration: &red_texture_config,
//                 },
//             )
//             .unwrap(),
//             joint_sprite: (
//                 oge::Sprite::new(oge::SpriteConfiguration {
//                     label: None,
//                     mesh: oge::SpriteMesh::new_elipse(12., 12., 16),
//                     texture: oge.create_texture(&red_texture_config).unwrap(),
//                 }),
//                 oge::Sprite::new(oge::SpriteConfiguration {
//                     label: None,
//                     mesh: oge::SpriteMesh::new_elipse(8., 8., 16),
//                     texture: oge
//                         .create_texture(&oge::TextureConfiguration {
//                             source: oge::TextureSource::Color(oge::Color::WHITE),
//                             ..red_texture_config
//                         })
//                         .unwrap(),
//                 }),
//             ),
//             handle: Joint {
//                 position: Vector2::new(0., 0.),
//                 velocity: Vector2::ZERO,
//             },
//             arms: [Arm::new(Joint::ZERO, 200.); 1].to_vec(),
//         }
//     }

//     fn update(&mut self, oge: &mut Oge) {
//         let cursor_position = oge.get_real_cursor_position();
//         self.handle.velocity = cursor_position.sub(&self.handle.position);
//         self.handle.position = cursor_position;
//         *self.curve.get_mut(0) = self.handle.position;

//         let mut pivot = &self.handle;
//         for arm in self.arms.iter_mut() {
//             arm.update(oge, &pivot);
//             pivot = &arm.joint;
//         }

//         for (i, point) in self.curve.points_mut().iter_mut().skip(1).enumerate() {
//             // println!("\n{}", i);
//             *point = self.arms[i].joint.position;
//         }
//     }

//     fn render(&mut self, oge: &mut Oge) {
//         oge.draw_sprites([self.curve.get_sprite()]);
//         // for point in self.curve.points().iter() {
//         //     self.joint_sprite.0.set_position(*point);
//         //     self.joint_sprite.1.set_position(*point);
//         //     oge.draw_sprites([&self.joint_sprite.0, &self.joint_sprite.1])
//         // }
//     }
// }

// fn main() -> oge::OgeResult<()> {
//     oge::main_loop::start([
//         //
//         Pendulum::load_script(),
//         load_window_handler!(1., 1.),
//     ])
// }

fn main() {}