use std::collections::HashSet;

use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::{
    common::{MainCamera, State, TimeState},
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
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        cursor_options.grab_mode = CursorGrabMode::None;
    }

    if state.editor {
        cursor_options.grab_mode = CursorGrabMode::None;
    }

    cursor_options.visible = cursor_options.grab_mode != CursorGrabMode::Locked;
}

pub fn handle_mouse_movement(
    cursor_options: Single<&CursorOptions>,
    mouse_buffer: Res<MouseBuffer>,
    mut query: Query<&mut CursorMove, With<Player>>,
    time: Res<Time>,
    state: Res<State>,
) {
    if state.editor || matches!(state.time, TimeState::Finished(_)) {
        for mut cursor_move in &mut query {
            cursor_move.0 = Vec2::ZERO;
        }
        return;
    }

    if cursor_options.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let delta = mouse_buffer.motion;

    if delta != Vec2::ZERO {
        for mut cursor_move in &mut query {
            cursor_move.0 += Vec2::new(delta.x, -delta.y) / 2. * time.delta_secs();
            if cursor_move.0.length() > 1. {
                cursor_move.0 = cursor_move.0.normalize();
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct InputBuffer {
    pressed: HashSet<KeyCode>,
    pub just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
}

pub fn grab_inputs(keys: Res<ButtonInput<KeyCode>>, mut buffer: ResMut<InputBuffer>) {
    for key in keys.get_just_pressed() {
        buffer.just_pressed.insert(*key);
    }
    for key in keys.get_just_released() {
        buffer.just_released.insert(*key);
    }

    buffer.pressed = keys.get_pressed().copied().collect();
}

pub fn clear_buffer(mut buffer: ResMut<InputBuffer>) {
    buffer.just_pressed.clear();
    buffer.just_released.clear();
}

#[derive(Resource, Default, Debug)]
pub struct MouseBuffer {
    just_pressed: HashSet<MouseButton>,
    just_released: HashSet<MouseButton>,
    pressed: HashSet<MouseButton>,
    motion: Vec2,
}

pub fn grab_mouse(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut buffer: ResMut<MouseBuffer>,
    accumulated_motion: Res<AccumulatedMouseMotion>,
) {
    for button in mouse_buttons.get_just_pressed() {
        buffer.just_pressed.insert(*button);
    }
    for button in mouse_buttons.get_just_released() {
        buffer.just_released.insert(*button);
    }

    buffer.pressed = mouse_buttons.get_pressed().copied().collect();

    buffer.motion += accumulated_motion.delta;
}

pub fn clear_mouse(mut buffer: ResMut<MouseBuffer>) {
    buffer.just_pressed.clear();
    buffer.just_released.clear();

    buffer.motion = Vec2::ZERO;
}
