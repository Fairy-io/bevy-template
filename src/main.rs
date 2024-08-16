use bevy::prelude::*;
use bevy_template::plugins::base_plugin::BasePlugin;

#[cfg(not(feature = "dev"))]
use lib::*;

#[cfg(feature = "dev")]
#[hot_lib_reloader::hot_module(dylib = "lib")]
mod lib_hot {
    use bevy::prelude::*;

    hot_functions_from_file!("lib/src/lib.rs");

    #[lib_updated]
    pub fn was_updated() -> bool {}

    pub use lib::Despawnable;
}

#[cfg(feature = "dev")]
use lib_hot::*;

fn main() {
    let mut app = App::default();

    app.add_plugins(DefaultPlugins)
        .add_plugins(BasePlugin)
        .add_systems(Startup, startup);

    #[cfg(feature = "dev")]
    {
        fn reload(mut commands: Commands, query: Query<Entity, With<Despawnable>>) {
            let updated = was_updated();

            if updated {
                for entity in query.iter() {
                    commands.entity(entity).despawn();
                }

                startup(commands);
            }
        }

        app.add_systems(Update, reload);
    }

    app.run();
}
