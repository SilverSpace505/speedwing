use bevy::prelude::*;

use crate::common::{
    CurrentLevel, FinishGizmoGroup, GameEntity, MainCamera, MovementGizmoGroup, SceneState, State,
    TimeState, dist_to_segment,
};
use crate::editor::Editor;
use crate::grid_map::{GridMap, manage_meshes};
use crate::input::{
    InputBuffer, MouseBuffer, clear_buffer, clear_mouse, grab_inputs, grab_mouse,
    handle_cursor_lock, handle_mouse_movement, touch_system,
};
use crate::particles::{Particles, ParticlesMaterial};
use crate::player::{CursorMove, Player};
use crate::render::{
    configure_gizmos, draw_dots, render_finish, render_movement, update_gizmo_config,
};

#[derive(Component)]
struct TimeText;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::Game), (Game::setup, configure_gizmos))
            .add_systems(OnExit(SceneState::Game), Game::cleanup)
            .init_gizmo_group::<MovementGizmoGroup>()
            .init_gizmo_group::<FinishGizmoGroup>()
            .add_systems(
                FixedUpdate,
                (
                    touch_system,
                    handle_mouse_movement,
                    //
                    manage_time,
                    //
                    Player::movement,
                    Player::apply_velocity,
                    check_finish,
                    //
                    clear_buffer,
                    clear_mouse,
                )
                    .chain()
                    .run_if(in_state(SceneState::Game)),
            )
            .add_systems(
                Update,
                (
                    grab_inputs,
                    grab_mouse,
                    //
                    Player::camera_follow,
                    // input
                    handle_cursor_lock,
                    Editor::camera_movement,
                    Editor::handle_mouse,
                    state_management,
                    // particles
                    Particles::update,
                    // render
                    manage_meshes,
                    draw_dots,
                    update_time_text,
                    update_gizmo_config,
                    render_movement,
                    render_finish,
                    Editor::render,
                    //
                    manage_exit,
                )
                    .run_if(in_state(SceneState::Game)),
            );
    }
}

impl Game {
    fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        current_level: Res<CurrentLevel>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ParticlesMaterial>>,
    ) {
        let mut grid_map = GridMap::new(10., 16, 0.5, true);

        commands.spawn((Camera2d::default(), MainCamera, GameEntity));

        if let Some(level) = &current_level.1.level {
            grid_map.load(level);
        }

        Player::spawn(
            current_level.1.start[0],
            current_level.1.start[1],
            current_level.1.start[2],
            &mut commands,
            &asset_server,
        );

        let particles = Particles::spawn_cmd(&mut commands, &mut meshes, &mut materials);

        commands.spawn((
            Text::new(""),
            TextFont {
                font_size: 32.,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.),
                left: Val::Px(10.),
                ..default()
            },
            TimeText,
            GameEntity,
        ));

        commands.insert_resource(particles);
        commands.insert_resource(grid_map);
        commands.insert_resource(State {
            debug: false,
            editor: false,
            time: TimeState::None,
            follow: 1.
        });
        commands.insert_resource(Editor::new());
        commands.insert_resource(TimeState::None);
        commands.init_resource::<InputBuffer>();
        commands.init_resource::<MouseBuffer>();
    }
    fn cleanup(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
        for entity in &query {
            commands.entity(entity).despawn();
        }

        commands.remove_resource::<GridMap>();
        commands.remove_resource::<State>();
        commands.remove_resource::<Editor>();
        commands.remove_resource::<Particles>();
        commands.remove_resource::<TimeState>();
        commands.remove_resource::<InputBuffer>();
        commands.remove_resource::<MouseBuffer>();
    }
}

fn manage_exit(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<SceneState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Backslash) {
        next_state.set(SceneState::Menu);
    }
}

fn check_finish(
    query: Query<&Transform, With<Player>>,
    current_level: Res<CurrentLevel>,
    mut state: ResMut<State>,
) {
    if !matches!(state.time, TimeState::Timing(_)) {
        return;
    }

    let Ok(transform) = query.single() else {
        return;
    };

    let Some(end) = current_level.1.end else {
        return;
    };

    let mut distance: Option<f32> = None;

    let end1 = Vec2::from_array(end[0]);
    let end2 = Vec2::from_array(end[1]);

    for point in Player::get_points(transform) {
        let d = dist_to_segment(point, end1, end2);
        distance = Some(match distance {
            Some(v) => v.min(d),
            None => d,
        })
    }

    let Some(distance) = distance else {
        return;
    };

    if distance < 50.
        && let TimeState::Timing(time) = state.time
    {
        state.time = TimeState::Finished(time);
    }
}

fn manage_time(mut state: ResMut<State>, query: Query<&CursorMove, With<Player>>, time: Res<Time>) {
    match state.time {
        TimeState::None => {
            let Ok(cursor_move) = query.single() else {
                return;
            };
            if cursor_move.0 != Vec2::ZERO {
                state.time = TimeState::Timing(0.);
            }
        }
        TimeState::Timing(current_time) => {
            state.time = TimeState::Timing(current_time + time.delta_secs())
        }
        TimeState::Finished(_) => (),
    };
}

fn state_management(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<State>,
    query: Query<Entity, With<Player>>,
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    asset_server: Res<AssetServer>,
) {
    if keyboard_input.just_pressed(KeyCode::Semicolon) {
        state.debug = !state.debug;
    }

    if keyboard_input.just_pressed(KeyCode::KeyR)
        && let Ok(entity) = query.single()
    {
        commands.entity(entity).despawn();

        Player::spawn(
            current_level.1.start[0],
            current_level.1.start[1],
            current_level.1.start[2],
            &mut commands,
            &asset_server,
        );

        state.time = TimeState::None;
    }
}

fn update_time_text(state: Res<State>, mut query: Query<&mut Text, With<TimeText>>) {
    if state.is_changed()
        && let Ok(mut text) = query.single_mut()
    {
        **text = format!(
            "{:.2}",
            match state.time {
                TimeState::None => 0.,
                TimeState::Timing(time) | TimeState::Finished(time) => time,
            }
        );
    }
}
