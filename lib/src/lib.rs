use bevy::prelude::*;

#[derive(Component)]
pub struct Despawnable;

#[no_mangle]
pub fn startup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section("Base plugin", TextStyle { ..default() }),
        Despawnable {},
    ));
}
