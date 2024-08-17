use bevy::prelude::*;

use crate::plugins::game_plugin::*;

pub fn hot_reload(mut commands: Commands, query: Query<Entity, With<Despawnable>>) {
    let updated = was_updated();

    if updated {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        startup(commands);
    }
}
