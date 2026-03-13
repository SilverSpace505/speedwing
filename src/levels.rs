use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::{
    common::{CurrentLevel, LevelData, SceneState},
    text_asset::TextAsset,
};

#[derive(Component)]
struct LevelEntity;

#[derive(Component)]
struct Level(u32);

#[derive(Resource)]
struct TextHandle(Handle<TextAsset>);

pub struct Levels;

impl Plugin for Levels {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::Levels), Levels::setup)
            .add_systems(OnExit(SceneState::Levels), Levels::cleanup)
            .add_systems(
                Update,
                (handle_level_buttons, check_load).run_if(in_state(SceneState::Levels)),
            );
    }
}

impl Levels {
    fn setup(mut commands: Commands) {
        commands.spawn((Camera2d, LevelEntity));

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.),
                    ..default()
                },
                LevelEntity,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Levels"),
                    TextFont {
                        font_size: 72.,
                        ..default()
                    },
                ));

                parent
                    .spawn(Node {
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(3, 1.),
                        grid_template_rows: RepeatedGridTrack::flex(1, 1.),
                        row_gap: Val::Px(10.),
                        column_gap: Val::Px(10.),
                        ..default()
                    })
                    .with_children(|grid| {
                        for i in 0..3 {
                            grid.spawn((
                                Button,
                                Node {
                                    width: Val::Px(60.),
                                    height: Val::Px(60.),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    border_radius: BorderRadius::all(Val::Px(10.)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0., 0.4, 0.8)),
                                Level(i),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new(format!("{}", i + 1)),
                                    TextFont {
                                        font_size: 32.,
                                        ..default()
                                    },
                                ));
                            });
                        }
                    });

                // parent
                //     .spawn((
                //         Button,
                //         Node {
                //             width: Val::Px(200.),
                //             height: Val::Px(65.),
                //             justify_content: JustifyContent::Center,
                //             align_items: AlignItems::Center,
                //             border_radius: BorderRadius::all(Val::Px(10.)),
                //             ..default()
                //         },
                //         BackgroundColor(Color::srgb(0., 0.4, 0.8)),

                //         PlayButton,
                //     ))
                //     .with_children(|parent| {
                //         parent.spawn((
                //             Text::new("Play"),
                //             TextFont {
                //                 font_size: 32.,
                //                 ..default()
                //             },
                //         ));
                //     });
            });
    }
    fn cleanup(mut commands: Commands, query: Query<Entity, With<LevelEntity>>) {
        for entity in &query {
            commands.entity(entity).despawn();
        }

        commands.remove_resource::<TextHandle>();
    }
}

fn handle_level_buttons(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &Level),
        (Changed<Interaction>, With<Level>),
    >,
    // mut next_state: ResMut<NextState<SceneState>>,
    mut cursor_options: Single<&mut CursorOptions>,
    mut current_level: ResMut<CurrentLevel>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    cursor_options.grab_mode = CursorGrabMode::None;
    cursor_options.visible = true;
    for (interaction, mut bg, level) in &mut query {
        match interaction {
            Interaction::Pressed => {
                current_level.0 = level.0;
                let handle = asset_server.load(format!("levels/{}.txt", level.0));
                commands.insert_resource(TextHandle(handle));
                // next_state.set(SceneState::Game);
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(0., 0.5, 0.9));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(0., 0.4, 0.8));
            }
        }
    }
}

fn check_load(
    mut current_level: ResMut<CurrentLevel>,
    handle: Option<Res<TextHandle>>,
    text_assets: Res<Assets<TextAsset>>,
    mut next_state: ResMut<NextState<SceneState>>,
) {
    let Some(handle) = handle else {
        return;
    };
    if let Some(text) = text_assets.get(&handle.0) {
        if let Ok(data) = serde_json::from_str::<LevelData>(&text.0) {
            current_level.1 = data
        } else {
            current_level.1 = LevelData {
                level: None,
                start: [0., 0., 0.],
                end: None
            }
        }
        next_state.set(SceneState::Game);
    }
}
