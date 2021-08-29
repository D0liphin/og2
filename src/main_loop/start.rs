use crate::*;
use winit::{
    dpi::{PhysicalSize, Size},
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

fn build_window() -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();

    let mut window_builder = winit::window::WindowBuilder::new();
    window_builder.window = WindowAttributes {
        min_inner_size: Some(Size::Physical(PhysicalSize {
            width: 16,
            height: 16,
        })),
        inner_size: Some(Size::Physical(PhysicalSize {
            width: 9 * (1 << 6),
            height: 9 * (1 << 6),
        })),
        ..Default::default()
    };
    let window = window_builder.build(&event_loop).unwrap();

    (event_loop, window)
}

pub fn start<const SCRIPT_COUNT: usize>(scripts: [fn(&Oge) -> Box<dyn DynScript>; SCRIPT_COUNT]) {
    let (event_loop, window) = build_window();
    env_logger::init();

    let mut oge = Oge {
        window_handler: WindowHandler::new(&window),
        render_state: RenderState::new(&window),
    };

    let mut scripts = scripts.map(|get_script| get_script(&oge));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(key_code),
                            state,
                            ..
                        },
                    ..
                } => {
                    // handle input
                }

                WindowEvent::Resized(physical_size) => {
                    oge.render_state.resize(&WindowDimensions::from(physical_size));
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // new_inner_size is &mut so w have to dereference it twice
                    oge.render_state.resize(&WindowDimensions::from(&**new_inner_size));
                }

                _ => {}
            },

            Event::RedrawRequested(_) => {
                for script in scripts.iter_mut() {
                    script.update(&oge);
                }
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => {}
        }
    });
}
