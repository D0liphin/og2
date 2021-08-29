use crate::*;

pub type LoadedScript = fn(&mut Oge) -> Box<dyn DynScript>;

pub trait Script: DynScript + Sized + 'static {
    /// Initialises this script, returning its starting state. This function is
    /// only executed one.
    fn start(oge: &mut Oge) -> Self;

    /// Called once, just after a frame has been rendered.
    fn update(&mut self, oge: &mut Oge) {}

    /// Called each time the window is resized or the scale factor has changed.  
    /// It is also called once, when the window is first initialised.
    fn window_resize(&mut self, oge: &mut Oge) {}

    fn load_script() -> LoadedScript {
        Self::get_boxed_dyn_script
    }

    fn get_boxed_dyn_script(oge: &mut Oge) -> Box<dyn DynScript> {
        Box::new(Self::start(oge))
    }
}

pub trait DynScript {
    fn update(&mut self, oge: &mut Oge);

    fn window_resize(&mut self, oge: &mut Oge);
}

impl<T: Script> DynScript for T {
    fn update(&mut self, oge: &mut Oge) {
        <Self as Script>::update(self, oge)
    }

    fn window_resize(&mut self, oge: &mut Oge) {
        <Self as Script>::window_resize(self, oge)
    }
}
