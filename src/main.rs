use bevy::prelude::*;
use bevy_template::plugins::{
    base_plugin::BasePlugin, dev_plugin::DevPlugin, game_plugin::startup,
};

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(BasePlugin)
        .add_plugins(DevPlugin)
        .add_systems(Startup, startup)
        .run();
}
