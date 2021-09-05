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
#[allow(unused_variables)]
pub trait Script: DynScript + Sized + 'static {
    fn start(oge: &mut Oge) -> Self;
    fn update(&mut self, oge: &mut Oge) {}
    fn render(&mut self, oge: &mut Oge) {}

    fn load_script() -> LoadedScript {
        Self::get_boxed_dyn_script
    }

    fn get_boxed_dyn_script(oge: &mut Oge) -> Box<dyn DynScript> {
        Box::new(Self::start(oge))
    }
}

pub trait DynScript {
    fn update(&mut self, oge: &mut Oge);
    fn render(&mut self, oge: &mut Oge);
}

impl<T: Script> DynScript for T {
    fn update(&mut self, oge: &mut Oge) {
        <Self as Script>::update(self, oge)
    }

    fn render(&mut self, oge: &mut Oge) {
        <Self as Script>::render(self, oge)
    }
}
