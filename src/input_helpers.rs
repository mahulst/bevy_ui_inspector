use bevy::prelude::*;
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Stable;
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Initializing;

// Prevent components that just got added immediately sending out a change event.
pub fn stabilize_inputs(initializing_q: Query<Entity, With<Initializing>>, mut commands: Commands) {
    for entity in initializing_q.iter() {
        commands
            .entity(entity)
            .remove::<Initializing>()
            .insert(Stable);
    }
}

