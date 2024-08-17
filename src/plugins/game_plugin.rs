#[cfg(not(feature = "dev"))]
pub use lib::*;

#[cfg(feature = "dev")]
#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    use bevy::prelude::*;

    hot_functions_from_file!("lib/src/lib.rs");

    #[lib_updated]
    pub fn was_updated() -> bool {}

    pub use lib::*;
}

#[cfg(feature = "dev")]
pub use hot_lib::*;
