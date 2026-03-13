use bevy::{math::bool, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Resource)]
pub enum TimeState {
    Finished(f32),
    Timing(f32),
    None
}

#[derive(Resource)]
pub struct State {
    pub debug: bool,
    pub editor: bool,
    pub time: TimeState,
    pub follow: f32
}

#[derive(Resource)]
pub struct CurrentLevel(pub u32, pub LevelData);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum SceneState {
    #[default]
    Menu,
    Levels,
    Game,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct GameEntity;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MovementGizmoGroup;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct FinishGizmoGroup;

#[derive(Serialize, Deserialize)]
pub struct LevelData {
    pub level: Option<String>,
    pub start: [f32; 3],
    pub end: Option<[[f32; 2]; 2]>
}

pub fn get_threshold() -> f32 {
   0.5
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

fn dist_to_segment_squared(p: Vec2, v: Vec2, w: Vec2) -> f32 {
    let l2 = v.distance_squared(w);
    if l2 == 0. {
        return p.distance_squared(v);
    }

    let t = ((p - v).dot(w - v) / l2).clamp(0., 1.);
    p.distance_squared(v + t * (w - v))
}

pub fn dist_to_segment(p: Vec2, v: Vec2, w: Vec2) -> f32 {
    dist_to_segment_squared(p, v, w).sqrt()
}