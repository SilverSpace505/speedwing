use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Resource)]
pub struct State {
    pub debug: bool,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MovementGizmoGroup;

pub fn get_threshold(elapsed: f32) -> f32 {
    // 0.5 + elapsed * 0.
    0.5 + (elapsed / 3.).sin() / 4.
}
