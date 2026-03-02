use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    common::{MainCamera, State},
    grid_map::GridMap,
};

#[derive(Resource)]
pub struct Editor {
    camera_vel: Vec2,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            camera_vel: Vec2::ZERO,
        }
    }
    pub fn update(
        mut state: ResMut<State>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut camera_query: Query<&mut Transform, With<MainCamera>>,
        time: Res<Time>,
        mut editor: ResMut<Editor>,
    ) {
        if keyboard_input.just_pressed(KeyCode::Quote) {
            state.editor = !state.editor;
        }

        if !state.editor {
            return;
        }

        let Ok(mut camera_transform) = camera_query.single_mut() else {
            return;
        };

        let camera_speed = 75.;

        if keyboard_input.pressed(KeyCode::KeyW) {
            editor.camera_vel.y += camera_speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            editor.camera_vel.y -= camera_speed * time.delta_secs();
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            editor.camera_vel.x -= camera_speed * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            editor.camera_vel.x += camera_speed * time.delta_secs();
        }

        editor.camera_vel = editor.camera_vel.lerp(Vec2::ZERO, time.delta_secs() * 10.);
        camera_transform.translation += editor.camera_vel.extend(0.);
    }
    pub fn handle_mouse(
        mut grid_map: ResMut<GridMap>,
        buttons: Res<ButtonInput<MouseButton>>,
        window: Single<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        state: Res<State>,
        time: Res<Time>,
    ) {
        if !state.editor {
            return;
        }

        let Ok((camera, camera_transform)) = q_camera.single() else {
            return;
        };

        let Some(position) = window.cursor_position() else {
            return;
        };

        let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, position) else {
            return;
        };

        let grid_scale = grid_map.scale();

        if buttons.pressed(MouseButton::Left) || buttons.pressed(MouseButton::Right) {
            let rx = (world_position.x / grid_scale).floor() as i32;
            let ry = (world_position.y / grid_scale).floor() as i32;
            let range: i32 = 10;

            let mut vs = Vec::new();
            for x in -range..(range + 1) {
                for y in -range..(range + 1) {
                    vs.push(grid_map.get(rx + x, ry + y).unwrap_or(0.));
                }
            }

            for x in -range..(range + 1) {
                for y in -range..(range + 1) {
                    let d = (((rx + x) as f32 * grid_scale - world_position.x).powi(2)
                        + ((ry + y) as f32 * grid_scale - world_position.y).powi(2))
                    .sqrt()
                        / grid_scale;
                    let xu = (x + range) as usize;
                    let yu = (y + range) as usize;

                    if buttons.pressed(MouseButton::Left) {
                        let v = 1. - d / range as f32;
                        if let Some(cv) = vs.get(xu * (range as usize * 2 + 1) + yu) {
                            grid_map.set(
                                rx + x,
                                ry + y,
                                cv.lerp(v.max(*cv), 30. * time.delta_secs()),
                            );
                        }
                    } else if buttons.pressed(MouseButton::Right) {
                        let v = d / range as f32;
                        if let Some(cv) = vs.get(xu * (range as usize * 2 + 1) + yu) {
                            grid_map.set(
                                rx + x,
                                ry + y,
                                cv.lerp(v.min(*cv), 30. * time.delta_secs()),
                            );
                        }
                    }
                }
            }
        }
    }
}
