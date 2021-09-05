#![feature(box_syntax, label_break_value)]
#![allow(unused_imports)]

macro_rules! usemod {
    [$($vis:vis $module:ident),* $(,)?] => {
        $(
            $vis mod $module;
            $vis use $module::*;
        )*
    }
}

usemod![
    pub math,
    pub main_loop,
    pub oge_controller,
    pub physics,
    pub sprite,
    pub error,
    pub(crate) render_state,
];

pub mod util;
pub(crate) use util::*;