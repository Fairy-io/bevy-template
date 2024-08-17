use bevy::prelude::*;

use crate::Despawnable;

#[no_mangle]
pub fn startup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section("Base plugin", TextStyle { ..default() }),
        Despawnable {},
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_text_exists() {
        let mut app = App::default();

        app.add_systems(Startup, startup);

        app.update();

        let world = app.world_mut();

        let text = world.query::<&Text>().iter(world).next().unwrap();

        assert_eq!(text.sections[0].value, "Base plugin");
    }
}
