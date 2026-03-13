mod common;
mod editor;
mod grid;
mod grid_map;
mod input;
mod player;
mod render;

mod game;
mod levels;
mod menu;

mod particles;
mod raycast;
mod text_asset;

use bevy::{
    asset::AssetMetaCheck, prelude::*, sprite_render::Material2dPlugin, window::WindowResolution,
};
use bevy_fix_cursor_unlock_web::FixPointerUnlockPlugin;
use bevy_transform_interpolation::prelude::TransformInterpolationPlugin;

use crate::{
    common::{CurrentLevel, LevelData, SceneState},
    game::Game,
    grid::GridMaterial,
    levels::Levels,
    menu::Menu,
    particles::ParticlesMaterial,
    text_asset::{TextAsset, TextAssetLoader},
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
        .add_plugins((
            FixPointerUnlockPlugin,
            TransformInterpolationPlugin::default(),
            Material2dPlugin::<GridMaterial>::default(),
            Material2dPlugin::<ParticlesMaterial>::default(),
        ))
        //
        .init_asset::<TextAsset>()
        .init_asset_loader::<TextAssetLoader>()
        .insert_resource(CurrentLevel(
            0,
            LevelData {
                level: None,
                start: [0., 0., 0.],
                end: None,
            },
        ))
        //
        .init_state::<SceneState>()
        .add_plugins(Menu)
        .add_plugins(Game)
        .add_plugins(Levels)
        //
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .insert_resource(Time::<Fixed>::from_hz(100.))
        .run();
}
