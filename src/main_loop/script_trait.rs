use crate::*;

pub type LoadedScript = fn(&mut Oge) -> Box<dyn DynScript>;

/// Initialises this script, returning its starting state. This function is
/// only executed one.
/// 
/// ```rs
/// fn start(oge: &mut Oge) -> Self;
/// ```
/// Called before the next frame render
/// 
/// ```rs
/// fn update(&mut self, oge: &mut Oge) {}
/// ```
/// Called after all scripts have had `update` called. Use this to render sprites
/// and textures. This will make renders occur with a longer input delay, but 
/// will make all views render at roughly the same time rahter than one by one.
/// 
/// ```rs
/// fn render(&self, oge: &mut Oge) {}
/// ```
/// Called each time the window is resized or the scale factor has changed.  
/// It is also called once, when the window is first initialised.
/// 
/// ```rs
/// fn window_resized(&mut self, oge: &mut Oge) {}
/// ```
/// Called each time the state of the keyboard changes, a.k.a a key is prssed or
/// released.   
///     
/// Check if a key is pressed with `Oge::get_key_down(&mut self, key_code: u8)`.
/// key code constatns are available under `oge::key_codes`.
/// 
/// ```rs
/// fn keyboard_input(&mut self, oge: &mut Oge) {}
/// ```
/// Called each time a mouse button is pressed or released
/// 
/// ```rs
/// fn mouse_input(&mut self, oge: &mut Oge) {}
/// ```
/// Called each time the cursor is moved
/// 
/// ```rs
/// fn cursor_moved(&mut self, oge: &mut Oge) {}
/// ```
pub trait Script: DynScript + Sized + 'static {
    fn start(oge: &mut Oge) -> Self;
    fn update(&mut self, oge: &mut Oge) {}
    fn render(&self, oge: &mut Oge) {}
    fn window_resized(&mut self, oge: &mut Oge) {}
    fn keyboard_input(&mut self, oge: &mut Oge) {}
    fn mouse_input(&mut self, oge: &mut Oge) {}
    fn cursor_moved(&mut self, oge: &mut Oge) {}

    fn load_script() -> LoadedScript {
        Self::get_boxed_dyn_script
    }

    fn get_boxed_dyn_script(oge: &mut Oge) -> Box<dyn DynScript> {
        Box::new(Self::start(oge))
    }
}

pub trait DynScript {
    fn update(&mut self, oge: &mut Oge);
    fn window_resized(&mut self, oge: &mut Oge);
    fn keyboard_input(&mut self, oge: &mut Oge);
    fn render(&self, oge: &mut Oge);
    fn mouse_input(&mut self, oge: &mut Oge);
    fn cursor_moved(&mut self, oge: &mut Oge);
}

impl<T: Script> DynScript for T {
    fn update(&mut self, oge: &mut Oge) {
        <Self as Script>::update(self, oge)
    }

    fn window_resized(&mut self, oge: &mut Oge) {
        <Self as Script>::window_resized(self, oge)
    }

    fn keyboard_input(&mut self, oge: &mut Oge) {
        <Self as Script>::keyboard_input(self, oge)
    }

    fn render(&self, oge: &mut Oge) {
        <Self as Script>::render(self, oge)
    }

    fn mouse_input(&mut self, oge: &mut Oge) {
        <Self as Script>::mouse_input(self, oge)
    }

    fn cursor_moved(&mut self, oge: &mut Oge) {
        <Self as Script>::cursor_moved(self, oge)
    }
}