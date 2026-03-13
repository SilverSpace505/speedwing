#[cfg(not(target_arch = "wasm32"))]
use arboard::Clipboard;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    common::{CurrentLevel, LevelData, MainCamera, State},
    grid_map::GridMap,
};

enum EndPhase {
    Start,
    End(Vec2),
}

#[derive(Resource)]
pub struct Editor {
    camera_vel: Vec2,
    end_phase: EndPhase,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            camera_vel: Vec2::ZERO,
            end_phase: EndPhase::Start,
        }
    }
    pub fn camera_movement(
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

        let camera_speed = 7500.;

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
        camera_transform.translation += editor.camera_vel.extend(0.) * time.delta_secs();
    }

    pub fn handle_mouse(
        mut grid_map: ResMut<GridMap>,
        buttons: Res<ButtonInput<MouseButton>>,
        window: Single<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        state: Res<State>,
        time: Res<Time>,
        mut editor: ResMut<Editor>,
        mut current_level: ResMut<CurrentLevel>,
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

        if keyboard_input.just_pressed(KeyCode::KeyG) {
            for x in -4..4 {
                for y in -4..4 {
                    grid_map.generate(x, y, 42, 0.05, (x as f64, y as f64));
                }
            }
        }

        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            Editor::modify_start_end(&buttons, &mut editor, &mut current_level, &world_position);
        } else {
            Editor::modify_level(&mut grid_map, &world_position, &buttons, &time);
        }

        if keyboard_input.just_pressed(KeyCode::KeyP) {
            let data = LevelData {
                level: grid_map.save().ok(),
                start: current_level.1.start,
                end: current_level.1.end,
            };

            if let Ok(save) = serde_json::to_string(&data) {
                #[cfg(not(target_arch = "wasm32"))]
                if let Ok(clipboard) = &mut Clipboard::new() {
                    clipboard.set_text(&save).ok();
                }
                
                info!(save);
            }
        }
    }

    pub fn modify_start_end(
        buttons: &ButtonInput<MouseButton>,
        editor: &mut Editor,
        current_level: &mut CurrentLevel,
        world_position: &Vec2,
    ) {
        if buttons.just_pressed(MouseButton::Left) {
            current_level.1.start = [world_position.x, world_position.y, 0.];
        }
        if buttons.just_pressed(MouseButton::Right) {
            match editor.end_phase {
                EndPhase::Start => editor.end_phase = EndPhase::End(world_position.clone()),
                EndPhase::End(start) => {
                    current_level.1.end =
                        Some([[start.x, start.y], [world_position.x, world_position.y]]);
                    editor.end_phase = EndPhase::Start
                }
            }
        }
    }
    pub fn modify_level(
        grid_map: &mut GridMap,
        world_position: &Vec2,
        buttons: &ButtonInput<MouseButton>,
        time: &Time,
    ) {
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
    pub fn render(
        mut gizmos: Gizmos,
        current_level: Res<CurrentLevel>,
        state: Res<State>,
        editor: Res<Editor>,
        window: Single<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        if !state.editor {
            return;
        }

        let level_data = &current_level.1;

        gizmos.circle_2d(
            Vec2::new(level_data.start[0], level_data.start[1]),
            5.,
            Color::linear_rgba(0., 1., 0., 0.8),
        );

        match editor.end_phase {
            EndPhase::Start => {
                if let Some(end) = level_data.end {
                    gizmos.line_2d(
                        Vec2::new(end[0][0], end[0][1]),
                        Vec2::new(end[1][0], end[1][1]),
                        Color::linear_rgba(1., 0., 0., 0.8),
                    );
                }
            }
            EndPhase::End(start) => {
                let Ok((camera, camera_transform)) = q_camera.single() else {
                    return;
                };

                let Some(position) = window.cursor_position() else {
                    return;
                };

                let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, position)
                else {
                    return;
                };
                gizmos.line_2d(start, world_position, Color::linear_rgba(1., 0., 0., 0.8));
            }
        }
    }
}
