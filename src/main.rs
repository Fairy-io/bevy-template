use bevy::prelude::*;
use bevy_template::plugins::base_plugin::BasePlugin;

#[cfg(not(feature = "dev"))]
use lib::*;

#[cfg(feature = "dev")]
#[hot_lib_reloader::hot_module(dylib = "lib")]
mod lib_hot {
    use bevy::prelude::*;

    hot_functions_from_file!("lib/src/lib.rs");
}

#[cfg(feature = "dev")]
use lib_hot::*;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(BasePlugin)
        .add_systems(Update, startup)
        .run();
}
