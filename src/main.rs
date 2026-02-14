use std::f32::consts::PI;

use bevy::{
    asset::{AssetMetaCheck, RenderAssetUsages},
    color::palettes::css::WHITE,
    input::mouse::AccumulatedMouseMotion,
    mesh::Indices,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, WindowResolution},
};
use bevy_fix_cursor_unlock_web::FixPointerUnlockPlugin;
use noise::{NoiseFn, Perlin};

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
        .init_gizmo_group::<MovementGizmoGroup>()
        .add_systems(Startup, (setup, configure_gizmos))
        .add_systems(
            Update,
            (
                player_movement,
                apply_velocity,
                camera_follow_player,
                touch_system,
                handle_cursor_lock,
                handle_mouse_movement,
                draw_dots,
                update_gizmo_config,
                render_movement,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity {
    value: Vec3,
}

#[derive(Component)]
struct CursorMove {
    value: Vec2,
}

#[derive(Component)]
struct MainCamera;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MovementGizmoGroup;

#[derive(Resource)]
struct Grid {
    x: f32,
    y: f32,
    width: usize,
    height: usize,
    spacing: f32,
    data: Vec<f32>,
}

impl Grid {
    fn new(x: f32, y: f32, width: usize, height: usize, spacing: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            spacing,
            data: vec![0.; width * height],
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<f32> {
        if x < self.width && y < self.height {
            Some(self.data[y * self.width + x])
        } else {
            None
        }
    }

    fn get_world(&self, x: f32, y: f32) -> Option<f32> {
        let gx = ((x - self.x) / self.spacing).floor();
        let gy = ((y - self.y) / self.spacing).floor();
        if gx < 0. || gy < 0. {
            return None;
        }
        self.get(gx as usize, gy as usize)
    }

    fn set(&mut self, x: usize, y: usize, v: f32) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = v.clamp(0., 1.)
        }
    }

    fn draw_dots(&self, mut gizmos: Gizmos) {
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(v) = self.get(x, y) {
                    let color = Color::linear_rgba(1., 1., 1., v);
                    gizmos.circle_2d(
                        Vec2::new(
                            self.x + x as f32 * self.spacing,
                            self.y + y as f32 * self.spacing,
                        ),
                        self.spacing / 5.,
                        color,
                    );
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut grid: ResMut<Grid>,
) {
    commands.spawn((Camera2d::default(), MainCamera));

    commands
        .spawn((
            Player,
            Velocity { value: Vec3::ZERO },
            CursorMove { value: Vec2::ZERO },
            Transform::from_scale(Vec3::new(0.25, 0.25, 1.)),
        ))
        .with_children(|parent| {
            parent.spawn(Sprite {
                image: asset_server.load("orb.png"),
                custom_size: Some(Vec2::new(100., 100.)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            });

            parent.spawn((
                Sprite {
                    image: asset_server.load("wing.png"),
                    custom_size: Some(Vec2::new(175., 375.)),
                    image_mode: SpriteImageMode::Auto,
                    ..default()
                },
                Transform::from_xyz(-77.5, 35., 0.),
            ));

            parent.spawn((
                Sprite {
                    image: asset_server.load("wing.png"),
                    custom_size: Some(Vec2::new(175., 375.)),
                    image_mode: SpriteImageMode::Auto,
                    flip_x: true,
                    ..default()
                },
                Transform::from_xyz(77.5, 35., 0.),
            ));
        });

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

    let perlin = Perlin::new(42);
    let scale = 0.05;

    for x in 0..grid.width {
        for y in 0..grid.height {
            let v = perlin.get([x as f64 * scale, y as f64 * scale]);
            let nv = (v + 1.) / 2.;
            grid.set(x, y, nv.powf(5.) as f32);
        }
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &CursorMove), With<Player>>,
    time: Res<Time>,
) {
    let Ok((mut velocity, cursor_move)) = query.single_mut() else {
        return;
    };

    let acceleration = 2000.0;
    let max_speed = 1000.0;
    let mut friction: f32 = 0.99;

    let mut direction = Vec3::ZERO;
    if cursor_move.value.length() >= 0.1 {
        direction += cursor_move.value.extend(0.);
    } else {
        friction = 0.95;
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if direction.length() > 0.0 {
        velocity.value += direction * acceleration * time.delta_secs();
    }

    velocity.value *= friction.powf(time.delta_secs() * 100.);

    if velocity.value.length() > max_speed {
        velocity.value = velocity.value.normalize() * max_speed;
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity, &CursorMove)>,
    time: Res<Time>,
    grid: Res<Grid>,
    // window_query: Query<&Window>,
) {
    // let Ok(window) = window_query.single() else {
    //     return;
    // };

    // let size = Vec2::new(window.width(), window.height());

    for (mut transform, velocity, cursor_move) in &mut query {
        transform.translation += velocity.value * time.delta_secs();

        if let Some(v) = grid.get_world(transform.translation.x, transform.translation.y) {
            transform.translation -= velocity.value * time.delta_secs() * v * 2.;
        }

        // if transform.translation.x > size.x / 2. {
        //     transform.translation.x = size.x / 2.;
        //     velocity.value.x *= -1.;
        // }
        // if transform.translation.x < -size.x / 2. {
        //     transform.translation.x = -size.x / 2.;
        //     velocity.value.x *= -1.;
        // }

        // if transform.translation.y > size.y / 2. {
        //     transform.translation.y = size.y / 2.;
        //     velocity.value.y *= -1.;
        // }
        // if transform.translation.y < -size.y / 2. {
        //     transform.translation.y = -size.y / 2.;
        //     velocity.value.y *= -1.;
        // }

        if cursor_move.value.length() < 0.1 {
            continue;
        }

        let target_angle = cursor_move.value.y.atan2(cursor_move.value.x) - PI / 2.;
        let target_rotation = Quat::from_rotation_z(target_angle);

        transform.rotation = transform.rotation.slerp(
            target_rotation,
            (1. - 0.002_f32.powf(time.delta_secs())) * (0.2 + cursor_move.value.length() * 1.3),
        );
    }
}

fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    camera_transform.translation = camera_transform.translation.lerp(
        player_transform.translation,
        1. - 0.01_f32.powf(time.delta_secs() * 3.),
    );
}

fn touch_system(
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

fn handle_cursor_lock(
    mut cursor_options: Single<&mut CursorOptions>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        cursor_options.grab_mode = CursorGrabMode::None;
    }

    cursor_options.visible = cursor_options.grab_mode != CursorGrabMode::Locked;
}

fn handle_mouse_movement(
    cursor_options: Single<&CursorOptions>,
    accumulated_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<&mut CursorMove, With<Player>>,
    time: Res<Time>,
) {
    if cursor_options.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let delta = accumulated_motion.delta;

    if delta != Vec2::ZERO {
        for mut cursor_move in &mut query {
            cursor_move.value += Vec2::new(delta.x, -delta.y) * time.delta_secs();
            if cursor_move.value.length() > 1. {
                cursor_move.value = cursor_move.value.normalize();
            }
        }
    }
}

fn configure_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<MovementGizmoGroup>();
    config.line.width = 8.;

    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 2.;
}

fn update_gizmo_config(
    mut config_store: ResMut<GizmoConfigStore>,
    mut query: Query<&CursorMove, With<Player>>,
) {
    let Ok(cursor_move) = query.single_mut() else {
        return;
    };

    let (config, _) = config_store.config_mut::<MovementGizmoGroup>();
    config.line.width = 8. * cursor_move.value.length();
}

fn render_movement(
    mut query: Query<(&Transform, &mut CursorMove), With<Player>>,
    mut gizmos: Gizmos<MovementGizmoGroup>,
) {
    let Ok((pos, cursor_move)) = query.single_mut() else {
        return;
    };

    let arrow_size = 100.;

    gizmos.line_2d(
        pos.translation.xy(),
        pos.translation.xy() + cursor_move.value * arrow_size,
        Color::linear_rgba(1., 1., 1., cursor_move.value.length() / 2.),
    );

    let cross = cursor_move.value.normalize_or_zero();
    let cross = Vec2::new(-cross.y, cross.x);

    let arrow = cursor_move.value.length() / 10.;

    gizmos.line_2d(
        pos.translation.xy() + cursor_move.value * arrow_size
            - cursor_move.value * arrow_size * arrow
            - cross * arrow_size * arrow,
        pos.translation.xy() + cursor_move.value * arrow_size,
        Color::linear_rgba(1., 1., 1., cursor_move.value.length() / 2.),
    );
    gizmos.line_2d(
        pos.translation.xy() + cursor_move.value * arrow_size
            - cursor_move.value * arrow_size * arrow
            + cross * arrow_size * arrow,
        pos.translation.xy() + cursor_move.value * arrow_size,
        Color::linear_rgba(1., 1., 1., cursor_move.value.length() / 2.),
    );
}

fn draw_dots(gizmos: Gizmos, grid: Res<Grid>) {
    grid.draw_dots(gizmos);
}
