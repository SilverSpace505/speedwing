mod common;
mod grid;
mod input;
mod player;
mod render;

use bevy::{
    asset::{AssetMetaCheck, RenderAssetUsages},
    color::palettes::css::WHITE,
    mesh::Indices,
    prelude::*,
    window::WindowResolution,
};
use bevy_fix_cursor_unlock_web::FixPointerUnlockPlugin;

use crate::{
    common::{MainCamera, MovementGizmoGroup, State},
    grid::Grid,
    input::{handle_cursor_lock, handle_mouse_movement, touch_system},
    player::Player,
    render::{configure_gizmos, draw_dots, render_movement, update_gizmo_config},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        resolution: WindowResolution::default(),
                        canvas: Some("#bevy-canvas".to_string()),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(FixPointerUnlockPlugin)
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .insert_resource(Grid::new(100., 100., 100, 100, 10.))
        .insert_resource(State { debug: false, moving: false })
        .init_gizmo_group::<MovementGizmoGroup>()
        .add_systems(Startup, (setup, configure_gizmos))
        .add_systems(
            Update,
            (
                // player
                Player::movement,
                Player::apply_velocity,
                Player::camera_follow,
                // input
                touch_system,
                handle_cursor_lock,
                handle_mouse_movement,
                // render
                draw_dots,
                update_gizmo_config,
                render_movement,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2d::default(), MainCamera));

    Player::spawn(&mut commands, &asset_server);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Px(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Speedwing"),
                TextFont {
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

    let positions = vec![
        [0.0, 0.0, 0.0],
        [100.0, 0.0, 0.0],
        [50.0, 100.0, 0.0],
        [100.0, 0.0, 0.0],
        [200.0, 0.0, 0.0],
        [150.0, 100.0, 0.0],
        [200.0, 0.0, 0.0],
        [300.0, 0.0, 0.0],
        [250.0, 100.0, 0.0],
    ];

    let colours = vec![
        [1.0, 0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
    ];

    let indices = Indices::U32(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);

    let mut mesh = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colours);
    mesh.insert_indices(indices);

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(Color::from(WHITE))),
        Transform::default(),
    ));
}
