#[macro_use]
macro_rules! usemod {
    [$($vis:vis $module:ident),*] => {
        $(
            $vis mod $module;
            $vis use $module::*;
        )*
    }
}

usemod![pub math];
