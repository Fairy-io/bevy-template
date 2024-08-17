use crate::plugins::game_plugin::*;
use bevy::prelude::*;

pub fn hot_reload(mut commands: Commands, query: Query<Entity, With<Despawnable>>) {
    let updated = was_updated();

    if updated {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        startup(commands);
    }
}
