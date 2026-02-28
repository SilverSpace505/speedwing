use bevy::{math::bool, prelude::*};

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Resource)]
pub struct State {
    pub debug: bool,
    pub moving: bool,
    pub editor: bool
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MovementGizmoGroup;

pub fn get_threshold(elapsed: f32, state: &State) -> f32 {
    match state.moving {
        true => 0.5 + (elapsed / 3.).sin() / 4.,
        false => 0.5,
    }
}

pub fn div_floor(a: i32, b: i32) -> i32 {
    let d = a / b;
    let r = a % b;
    if (r != 0) && ((r < 0) != (b < 0)) {
        d - 1
    } else {
        d
    }
}

pub fn in_viewport(point: Vec2, camera: &Camera, camera_transform: &GlobalTransform) -> bool {
    if let Ok(viewport_pos) = camera.world_to_viewport(camera_transform, point.extend(0.)) {
        if let Some(viewport_size) = camera.logical_viewport_size() {
            return viewport_pos.x >= 0.
                && viewport_pos.y >= 0.
                && viewport_pos.x <= viewport_size.x
                && viewport_pos.y <= viewport_size.y;
        }
    }
    false
}