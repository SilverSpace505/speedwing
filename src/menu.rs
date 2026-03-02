use bevy::{prelude::*, window::{CursorGrabMode, CursorOptions}};

use crate::common::SceneState;

#[derive(Component)]
struct MenuEntity;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::Menu), Menu::setup)
            .add_systems(OnExit(SceneState::Menu), Menu::cleanup)
            .add_systems(
                Update,
                handle_play_button.run_if(in_state(SceneState::Menu)),
            );
    }
}

impl Menu {
    fn setup(mut commands: Commands) {
        commands.spawn((Camera2d, MenuEntity));

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
                MenuEntity,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Speedwing"),
                    TextFont {
                        font_size: 72.,
                        ..default()
                    },
                ));

                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.),
                            height: Val::Px(65.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(10.)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0., 0.4, 0.8)),
                        
                        PlayButton,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Play"),
                            TextFont {
                                font_size: 32.,
                                ..default()
                            },
                        ));
                    });
            });
    }
    fn cleanup(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
struct PlayButton;

fn handle_play_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<SceneState>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    cursor_options.grab_mode = CursorGrabMode::None;
    cursor_options.visible = true;
    for (interaction, mut bg) in &mut query {
        match interaction {
            Interaction::Pressed => {
                next_state.set(SceneState::Game);
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