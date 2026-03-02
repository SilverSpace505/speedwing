mod common;
mod editor;
mod grid;
mod grid_map;
mod input;
mod player;
mod render;

mod game;
mod menu;

use bevy::{
    asset::AssetMetaCheck, prelude::*, sprite_render::Material2dPlugin, window::WindowResolution,
};
use bevy_fix_cursor_unlock_web::FixPointerUnlockPlugin;

use crate::{common::SceneState, game::Game, grid::GridMaterial, menu::Menu};

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
        .add_plugins((
            FixPointerUnlockPlugin,
            Material2dPlugin::<GridMaterial>::default(),
        ))
        //
        .init_state::<SceneState>()
        .add_plugins(Menu)
        .add_plugins(Game)
        //
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .run();
}