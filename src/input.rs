use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::{
    common::{MainCamera, State},
    player::{CursorMove, Player},
};

pub fn touch_system(
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    touches: Res<Touches>,
) {
    let Ok(mut transform) = player_query.single_mut() else {
        return;
    };

    let Ok((camera, camera_global_transform)) = camera_query.single() else {
        return;
    };

    for touch in touches.iter() {
        if let Ok(world_position) =
            camera.viewport_to_world_2d(camera_global_transform, touch.position())
        {
            transform.translation = world_position.extend(0.);
        }
    }
}

pub fn handle_cursor_lock(
    mut cursor_options: Single<&mut CursorOptions>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        cursor_options.grab_mode = CursorGrabMode::None;
    }

    if state.editor {
        cursor_options.grab_mode = CursorGrabMode::None;
    }

    cursor_options.visible = cursor_options.grab_mode != CursorGrabMode::Locked;
}

pub fn handle_mouse_movement(
    cursor_options: Single<&CursorOptions>,
    accumulated_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<&mut CursorMove, With<Player>>,
    time: Res<Time>,
    state: Res<State>,
) {
    if state.editor {
        for mut cursor_move in &mut query {
            cursor_move.0 = Vec2::ZERO;
        }
    }

    if cursor_options.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let delta = accumulated_motion.delta;

    if delta != Vec2::ZERO {
        for mut cursor_move in &mut query {
            cursor_move.0 += Vec2::new(delta.x, -delta.y) / 2. * time.delta_secs();
            if cursor_move.0.length() > 1. {
                cursor_move.0 = cursor_move.0.normalize();
            }
        }
    }
}
