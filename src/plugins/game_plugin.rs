use bevy::prelude::*;

#[cfg(not(feature = "dev"))]
use lib::*;

#[cfg(feature = "dev")]
#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    use bevy::prelude::*;

    hot_functions_from_file!("lib/src/lib.rs");

    hot_functions_from_file!("lib/src/components/despawnable.rs");

    hot_functions_from_file!("lib/src/systems/startup_system.rs");
    hot_functions_from_file!("lib/src/systems/despawn_scene_system.rs");

    #[lib_updated]
    pub fn was_updated() -> bool {}

    pub use lib::*;
}

#[cfg(feature = "dev")]
use hot_lib::*;

#[cfg(feature = "dev")]
pub use hot_lib::startup;

#[cfg(feature = "dev")]
pub use hot_lib::Despawnable;

#[cfg(feature = "dev")]
pub use hot_lib::was_updated;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, despawn_scene);
    }
}
