use bevy::prelude::*;
use bevy_template::plugins::base_plugin::BasePlugin;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(BasePlugin)
        .run();
}
