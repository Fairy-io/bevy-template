use bevy::prelude::*;

use crate::Despawnable;

#[no_mangle]
pub fn startup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section("Base plugin", TextStyle { ..default() }),
        Despawnable {},
    ));
}
