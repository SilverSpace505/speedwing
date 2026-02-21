use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Resource)]
pub struct State {
    pub debug: bool,
    pub moving: bool
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MovementGizmoGroup;

pub fn get_threshold(elapsed: f32, state: &State) -> f32 {
    match state.moving {
        true => 0.5 + (elapsed / 3.).sin() / 4.,
        false => 0.5
    }
}
