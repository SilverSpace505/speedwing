use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    common::{GameEntity, MainCamera, State, Velocity, get_threshold},
    grid_map::GridMap,
};

#[derive(Component)]
pub struct Player {
    pub normal: Vec2,
}

const POINTS: [(f32, f32); 6] = [
    (0., 0.),
    (-125., -125.),
    (125., -125.),
    (0., 200.),
    (-95., 25.),
    (95., 25.),
];

impl Player {
    pub fn spawn(commands: &mut Commands<'_, '_>, asset_server: &Res<AssetServer>) {
        commands.spawn(Player::bundle()).with_children(|parent| {
            parent.spawn(Sprite {
                image: asset_server.load("orb.png"),
                custom_size: Some(Vec2::new(100., 100.)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            });

            parent.spawn((
                Sprite {
                    image: asset_server.load("wing.png"),
                    custom_size: Some(Vec2::new(160., 360.)),
                    image_mode: SpriteImageMode::Auto,
                    ..default()
                },
                Transform::from_xyz(-75., 45., 0.),
            ));

            parent.spawn((
                Sprite {
                    image: asset_server.load("wing.png"),
                    custom_size: Some(Vec2::new(160., 360.)),
                    image_mode: SpriteImageMode::Auto,
                    flip_x: true,
                    ..default()
                },
                Transform::from_xyz(75., 45., 0.),
            ));
        });
    }
    fn bundle() -> impl Bundle {
        (
            Self { normal: Vec2::ZERO },
            Velocity(Vec3::ZERO),
            CursorMove(Vec2::ZERO),
            Transform::from_scale(Vec3::new(0.25, 0.25, 1.)),
            GameEntity
        )
    }
    pub fn movement(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut query: Query<(&mut Player, &mut Velocity, &CursorMove, &Transform), With<Player>>,
        time: Res<Time>,
        grid_map: Res<GridMap>,
        mut state: ResMut<State>,
    ) {
        // grid.generate(
        //     42,
        //     0.05,
        //     match state.moving {
        //         true => (
        //             (time.elapsed_secs() / 5.).into(),
        //             (time.elapsed_secs() / 10.).into(),
        //         ),
        //         false => (0., 0.),
        //     },
        // );
        // if state.moving {
        //     grid.set_mesh(
        //         &mut meshes,
        //         grid.gen_attributes(get_threshold(time.elapsed_secs(), &state), true),
        //     );
        // }

        let Ok((mut player, mut velocity, cursor_move, transform)) = query.single_mut() else {
            return;
        };

        if let Some(normal) =
            grid_map.get_normal_world(transform.translation.x, transform.translation.y)
        {
            player.normal = Vec2::new(normal.0, normal.1);
        }

        let acceleration = 2000.0;
        let max_speed = 1000.0;
        let mut friction: f32 = 0.99;

        // let editor = state.editor;

        let mut direction = Vec3::ZERO;
        if cursor_move.0.length() >= 0.1 {
            direction += cursor_move.0.extend(0.);
        } else {
            friction = 0.95;
        }

        // if keyboard_input.pressed(KeyCode::KeyW) && !editor {
        //     direction.y += 1.0;
        // }
        // if keyboard_input.pressed(KeyCode::KeyS) && !editor {
        //     direction.y -= 1.0;
        // }

        // if keyboard_input.pressed(KeyCode::KeyD) && !editor {
        //     direction.x += 1.0;
        // }
        // if keyboard_input.pressed(KeyCode::KeyA) && !editor {
        //     direction.x -= 1.0;
        // }

        if keyboard_input.just_pressed(KeyCode::Semicolon) {
            state.debug = !state.debug;
        }

        if keyboard_input.just_pressed(KeyCode::KeyM) {
            state.moving = !state.moving;
            // if !state.moving {
            //     grid.generate(
            //         42,
            //         0.05,
            //         match state.moving {
            //             true => (
            //                 (time.elapsed_secs() / 5.).into(),
            //                 (time.elapsed_secs() / 10.).into(),
            //             ),
            //             false => (0., 0.),
            //         },
            //     );
            //     grid.set_mesh(
            //         &mut meshes,
            //         grid.gen_attributes(get_threshold(time.elapsed_secs(), &state), true),
            //     );
            // }
        }

        if direction.length() > 0.0 {
            velocity.0 += direction * acceleration * time.delta_secs();
        }

        velocity.0 *= friction.powf(time.delta_secs() * 100.);

        if velocity.0.length() > max_speed {
            velocity.0 = velocity.0.normalize() * max_speed;
        }
    }
    fn is_colliding(
        &self,
        grid_map: &Res<GridMap>,
        threshold: f32,
        transform: &Transform,
    ) -> Option<Vec3> {
        let mut normal = Vec3::ZERO;
        for offset in POINTS {
            let offset = transform.transform_point(Vec3::new(offset.0, offset.1, 0.));
            if grid_map
                .get_world(offset.x, offset.y)
                .is_some_and(|v| v > threshold + 0.02)
                && let Some(pnormal) = grid_map.get_normal_world(offset.x, offset.y)
            {
                normal += Vec3::new(pnormal.0, pnormal.1, 0.);
            }
        }
        match normal.length() > 0. {
            true => Some(normal.normalize_or_zero()),
            false => None,
        }
    }
    pub fn apply_velocity(
        mut query: Query<(&Player, &mut Transform, &mut Velocity, &CursorMove)>,
        time: Res<Time>,
        grid_map: Res<GridMap>,
        state: Res<State>,
    ) {
        let threshold = get_threshold(time.elapsed_secs(), &state);
        let Ok((player, mut transform, mut velocity, cursor_move)) = query.single_mut() else {
            return;
        };

        transform.translation += velocity.0 * time.delta_secs();

        if let Some(normal) = player.is_colliding(&grid_map, threshold, &transform) {
            transform.translation -= velocity.0 * time.delta_secs();

            let mut dir = 0.05;

            while let Some(normal) = player.is_colliding(&grid_map, threshold, &transform) {
                transform.translation -= normal * dir;
                dir = -dir - 0.05 * dir.signum();
                transform.translation += normal * dir;
            }

            let change = -velocity.0.dot(normal) * normal;
            velocity.0 = velocity.0 + change;

            transform.translation += velocity.0 * time.delta_secs();
        }

        if cursor_move.0.length() < 0.1 {
            return;
        }

        let target_angle = cursor_move.0.y.atan2(cursor_move.0.x) - PI / 2.;
        let target_rotation = Quat::from_rotation_z(target_angle);

        transform.rotation = transform.rotation.slerp(
            target_rotation,
            (1. - 0.002_f32.powf(time.delta_secs())) * (0.2 + cursor_move.0.length() * 1.3),
        );
    }
    pub fn camera_follow(
        player_query: Query<(&Transform, &Velocity), (With<Player>, Without<MainCamera>)>,
        mut camera_query: Query<&mut Transform, With<MainCamera>>,
        time: Res<Time>,
        state: Res<State>,
    ) {
        if state.editor {
            return;
        }
        let Ok((player_transform, velocity)) = player_query.single() else {
            return;
        };

        let Ok(mut camera_transform) = camera_query.single_mut() else {
            return;
        };

        camera_transform.translation = camera_transform.translation.lerp(
            player_transform.translation,
            1. - 0.01_f32.powf(time.delta_secs() * 3.),
        );

        camera_transform.scale = camera_transform.scale.lerp(
            Vec3::splat(1. + 0.0002 * velocity.0.length()),
            1. - 0.01_f32.powf(time.delta_secs()),
        );
    }
    pub fn draw_points(&self, gizmos: &mut Gizmos, transform: &Transform) {
        for offset in POINTS {
            let offset = transform.transform_point(Vec3::new(offset.0, offset.1, 0.));
            gizmos.circle_2d(
                Vec2::new(offset.x, offset.y),
                2.5,
                Color::linear_rgba(1., 0., 0., 0.8),
            );
        }
    }
}

#[derive(Component)]
pub struct CursorMove(pub Vec2);
