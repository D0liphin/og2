use crate::*;

pub trait Script: DynScript + Sized + 'static {
    /// Initialises this script, returning its starting state. This function is
    /// only executed one.
    fn start(oge: &Oge) -> Self;

    /// Called once, just after a frame has been rendered.
    fn update(&mut self, oge: &Oge);

    fn load_script(oge: &Oge) -> Box<dyn DynScript> {
        Box::new(Self::start(oge))
    }
}

pub trait DynScript {
    fn update(&mut self, oge: &Oge);
}

impl<T: Script> DynScript for T {
    fn update(&mut self, oge: &Oge) {
        <Self as Script>::update(self, oge)
    }
}
