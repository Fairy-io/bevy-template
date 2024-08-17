use bevy::prelude::*;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, _app: &mut App) {
        #[cfg(feature = "dev")]
        {
            use crate::systems::hot_reload_system::hot_reload;

            _app.add_systems(Update, hot_reload);
        }
    }
}
