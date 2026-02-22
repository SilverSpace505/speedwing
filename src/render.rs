use bevy::prelude::*;

use crate::{
    common::{MovementGizmoGroup, State, Velocity, get_threshold},
    grid::Grid,
    player::{CursorMove, Player},
};

pub fn configure_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<MovementGizmoGroup>();
    config.line.width = 8.;

    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 2.;
}

pub fn update_gizmo_config(
    mut config_store: ResMut<GizmoConfigStore>,
    mut query: Query<&CursorMove, With<Player>>,
) {
    let Ok(cursor_move) = query.single_mut() else {
        return;
    };

    let (config, _) = config_store.config_mut::<MovementGizmoGroup>();
    config.line.width = 8. * cursor_move.0.length();
}

pub fn render_movement(
    mut query: Query<(&Player, &Transform, &mut CursorMove, &Velocity), With<Player>>,
    mut movement_gizmos: Gizmos<MovementGizmoGroup>,
    mut gizmos: Gizmos,
    state: Res<State>,
) {
    let Ok((player, pos, cursor_move, velocity)) = query.single_mut() else {
        return;
    };

    draw_arrow(
        &mut movement_gizmos,
        pos.translation.xy(),
        cursor_move.0 * 100.,
        10.,
        cursor_move.0.length(),
        Color::linear_rgba(1., 1., 1., cursor_move.0.length() / 2.),
    );

    if state.debug {
        draw_arrow(
            &mut movement_gizmos,
            pos.translation.xy(),
            player.normal * 100.,
            10.,
            1.,
            Color::linear_rgba(0., 0., 1., 0.8),
        );

        draw_arrow(
            &mut movement_gizmos,
            pos.translation.xy(),
            velocity.0.xy() / 10.,
            10.,
            velocity.0.length() / 1000.,
            Color::linear_rgba(0., 1., 0., velocity.0.length() / 1000.),
        );

        player.draw_points(&mut gizmos, &pos);
    }
}

pub fn draw_arrow(
    gizmos: &mut Gizmos<MovementGizmoGroup>,
    start: Vec2,
    vec: Vec2,
    arrow_size: f32,
    arrow: f32,
    color: Color,
) {
    gizmos.line_2d(start, start + vec, color);

    let norm = vec.normalize_or_zero();
    let cross = Vec2::new(-norm.y, norm.x);

    gizmos.line_2d(
        start + vec - arrow_size * arrow * norm - cross * arrow_size * arrow,
        start + vec,
        color,
    );
    gizmos.line_2d(
        start + vec - arrow_size * arrow * norm + cross * arrow_size * arrow,
        start + vec,
        color,
    );
}

pub fn draw_dots(mut gizmos: Gizmos, grid: Res<Grid>, time: Res<Time>, state: Res<State>) {
    if state.debug {
        grid.draw_dots(&mut gizmos);
        grid.draw_segments(get_threshold(time.elapsed_secs(), &state), true, &mut gizmos);
    }
}