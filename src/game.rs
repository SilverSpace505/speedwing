
use bevy::prelude::*;

use crate::common::{GameEntity, MainCamera, MovementGizmoGroup, SceneState, State};
use crate::editor::Editor;
use crate::grid_map::{GridMap, manage_meshes};
use crate::input::{handle_cursor_lock, handle_mouse_movement, touch_system};
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
        // mut meshes: ResMut<Assets<Mesh>>,
        // mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let mut grid_map = GridMap::new(10., 16, 0.5, true);

        commands.spawn((Camera2d::default(), MainCamera, GameEntity));

        Player::spawn(&mut commands, &asset_server);

        // commands
        //     .spawn((
        //         Node {
        //             width: Val::Percent(100.0),
        //             height: Val::Px(100.0),
        //             justify_content: JustifyContent::Center,
        //             align_items: AlignItems::Center,
        //             ..default()
        //         },
        //         GameEntity,
        //     ))
        //     .with_children(|parent| {
        //         parent.spawn((
        //             Text::new("Speedwing"),
        //             TextFont {
        //                 font_size: 25.0,
        //                 ..default()
        //             },
        //             TextColor(Color::WHITE),
        //             GameEntity,
        //         ));
        //     });

        // let positions = vec![
        //     [0.0, 0.0, 0.0],
        //     [100.0, 0.0, 0.0],
        //     [50.0, 100.0, 0.0],
        //     [100.0, 0.0, 0.0],
        //     [200.0, 0.0, 0.0],
        //     [150.0, 100.0, 0.0],
        //     [200.0, 0.0, 0.0],
        //     [300.0, 0.0, 0.0],
        //     [250.0, 100.0, 0.0],
        // ];

        // let colours = vec![
        //     [1.0, 0.0, 0.0, 1.0],
        //     [1.0, 0.0, 0.0, 1.0],
        //     [1.0, 0.0, 0.0, 1.0],
        //     [0.0, 1.0, 0.0, 1.0],
        //     [0.0, 1.0, 0.0, 1.0],
        //     [0.0, 1.0, 0.0, 1.0],
        //     [0.0, 0.0, 1.0, 1.0],
        //     [0.0, 0.0, 1.0, 1.0],
        //     [0.0, 0.0, 1.0, 1.0],
        // ];

        // let indices = Indices::U32(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);

        // let mut mesh = Mesh::new(
        //     bevy::mesh::PrimitiveTopology::TriangleList,
        //     RenderAssetUsages::default(),
        // );
        // mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colours);
        // mesh.insert_indices(indices);

        // commands.spawn((
        //     Mesh2d(meshes.add(mesh)),
        //     MeshMaterial2d(materials.add(Color::from(WHITE))),
        //     Transform::default(),
        //     GameEntity,
        // ));

        for x in -4..4 {
            for y in -4..4 {
                grid_map.generate(x, y, 42, 0.05, (x as f64, y as f64));
            }
        }

        commands.insert_resource(grid_map);
        commands.insert_resource(State {
            debug: false,
            moving: false,
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
    }
}

fn manage_exit(keyboard_input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<SceneState>>) {
    if keyboard_input.just_pressed(KeyCode::Backslash) {
        next_state.set(SceneState::Menu);
    }
}