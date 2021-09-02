use crate::*;
use winit::{
    dpi::{PhysicalSize, Size},
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

fn build_window() -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();

    let icon = image::load_from_memory(include_bytes!("../images/logo.png")).unwrap();
    let icon_bytes = icon.as_rgba8().unwrap();

    let mut window_builder = winit::window::WindowBuilder::new();
    window_builder.window = WindowAttributes {
        title: "Oge Window".to_owned(),
        window_icon: Some(winit::window::Icon::from_rgba(icon_bytes.to_vec(), 64, 64).unwrap()),
        min_inner_size: Some(Size::Physical(PhysicalSize {
            width: 16,
            height: 16,
        })),
        inner_size: Some(Size::Physical(PhysicalSize {
            width: 16 * (1 << 6),
            height: 9 * (1 << 6),
        })),
        ..Default::default()
    };
    let window = window_builder.build(&event_loop).unwrap();

    (event_loop, window)
}

pub fn start<const SCRIPT_COUNT: usize>(
    scripts: [fn(&mut Oge) -> Box<dyn DynScript>; SCRIPT_COUNT],
) -> Result<(), OgeError> {
    let (event_loop, window) = build_window();
    env_logger::init();

    let mut render_state = RenderState::new(&window);
    let mut oge_handlers = OgeHandlers {
        window_handler: WindowHandler::new(&window),
        input_handler: InputHandler::new(),
        meta_handler: MetaHandler::new(),
    };

    let dyn_scripts: [Box<dyn DynScript>; SCRIPT_COUNT];
    {
        let mut render_pass_resources = render_state.create_render_pass_resources()?;
        let mut oge = Oge::new(
            &mut oge_handlers,
            &mut render_state,
            &mut render_pass_resources,
        );
        dyn_scripts = scripts.map(|get_script| get_script(&mut oge));
    }
    let mut scripts = dyn_scripts;

    event_loop.run(move |event, _, control_flow| 'event_handler: {
        //
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
                    let key_code = *key_code as u32 as usize;
                    oge_handlers.input_handler.set_keyboard_input_state(
                        key_code,
                        if *state == ElementState::Pressed {
                            ButtonState::Pressed
                        } else {
                            ButtonState::Released
                        },
                    );
                }

                WindowEvent::MouseInput { button, state, .. } => {
                    let mouse_button_code = match *button {
                        MouseButton::Left => 0,
                        MouseButton::Right => 1,
                        MouseButton::Middle => 2,
                        MouseButton::Other(n) => n as usize,
                    };
                    oge_handlers.input_handler.set_mouse_input_state(
                        mouse_button_code,
                        if *state == ElementState::Pressed {
                            ButtonState::Pressed
                        } else {
                            ButtonState::Released
                        },
                    );
                }

                WindowEvent::CursorMoved { position, .. } => {
                    oge_handlers
                        .input_handler
                        .set_cursor_physical_position(position);
                }

                WindowEvent::Resized(physical_size) => {
                    let window_dimensions = WindowDimensions::from(physical_size);
                    render_state.resize(&window_dimensions);
                    oge_handlers.window_handler.resize(window_dimensions);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    let window_dimensions = WindowDimensions::from(&**new_inner_size);
                    render_state.resize(&window_dimensions);
                    oge_handlers.window_handler.resize(window_dimensions);
                }

                _ => {}
            },

            Event::RedrawRequested(_) => {
                let mut render_pass_resources = match render_state.create_render_pass_resources() {
                    Ok(resources) => resources,
                    Err(_) => break 'event_handler,
                };
                let mut oge = Oge::new(
                    &mut oge_handlers,
                    &mut render_state,
                    &mut render_pass_resources,
                );

                oge.handlers.meta_handler.update();
                for script in scripts.iter_mut() {
                    script.update(&mut oge);
                }
                for script in scripts.iter_mut() {
                    script.render(&mut oge);
                }
                oge.handlers.input_handler.update();

                let mut render_pass = oge.finish();
                render_pass.draw_render_bundles(&mut render_state);
                render_pass_resources.finish(&render_state);
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => {}
        }
    });
}
