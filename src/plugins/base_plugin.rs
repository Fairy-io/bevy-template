use bevy::prelude::*;
pub struct BasePlugin;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}
