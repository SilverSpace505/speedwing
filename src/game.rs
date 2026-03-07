
use bevy::prelude::*;

use crate::common::{CurrentLevel, GameEntity, MainCamera, MovementGizmoGroup, SceneState, State};
use crate::editor::Editor;
use crate::grid_map::{GridMap, manage_meshes};
use crate::input::{handle_cursor_lock, handle_mouse_movement, touch_system};
use crate::particles::{Particles, ParticlesMaterial};
use crate::player::Player;
use crate::render::{configure_gizmos, draw_dots, render_movement, update_gizmo_config};

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::Game), (Game::setup, configure_gizmos))
            .add_systems(OnExit(SceneState::Game), Game::cleanup)
            .init_gizmo_group::<MovementGizmoGroup>()
            .add_systems(
                Update,
                (
                    Player::movement,
                    Player::apply_velocity,
                    Player::camera_follow,
                    // input
                    touch_system,
                    handle_cursor_lock,
                    handle_mouse_movement,
                    Editor::update,
                    Editor::handle_mouse,
                    // particles
                    Particles::update,
                    // render
                    manage_meshes,
                    draw_dots,
                    update_gizmo_config,
                    render_movement,
                    //
                    manage_exit
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
        mut materials: ResMut<Assets<ParticlesMaterial>>
    ) {
        let mut grid_map = GridMap::new(10., 16, 0.5, true);

        commands.spawn((Camera2d::default(), MainCamera, GameEntity));

        Player::spawn(&mut commands, &asset_server);

        if let Some(text) = &current_level.1 {
            grid_map.load(text);
        }

        let particles = Particles::spawn_cmd(&mut commands, &mut meshes, &mut materials);

        commands.insert_resource(particles);
        commands.insert_resource(grid_map);
        commands.insert_resource(State {
            debug: false,
            editor: false,
        });
        commands.insert_resource(Editor::new());
    }
    fn cleanup(mut commands: Commands, query: Query<Entity, With<GameEntity>>) {
        for entity in &query {
            commands.entity(entity).despawn();
        }

        commands.remove_resource::<GridMap>();
        commands.remove_resource::<State>();
        commands.remove_resource::<Editor>();
        commands.remove_resource::<Particles>();
    }
}

fn manage_exit(keyboard_input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<SceneState>>) {
    if keyboard_input.just_pressed(KeyCode::Backslash) {
        next_state.set(SceneState::Menu);
    }
}