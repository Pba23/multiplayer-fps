#[derive(Component)]
pub struct FpsText;

use bevy::prelude::*;

#[derive(Component)]
struct GameOverText;

pub fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 25.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 25.0,
                color: Color::GOLD,
            }),
            TextSection::from_style(TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 0.0,
                color: Color::GOLD,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect { top: Val::Px(10.0), left: Val::Px(200.0), ..default() },
            ..default()
        }),
        FpsText,
    ));
}
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

pub fn update_fps_text(mut query: Query<&mut Text, With<FpsText>>, diagnostics: Res<Diagnostics>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            if let Ok(mut text) = query.get_single_mut() {
                text.sections[2].value = format!("{:.2}", average);
            }
        }
    }
}

pub fn display_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Game Over",
            TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::RED,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(300.0),
                left: Val::Px(400.0),
                ..default()
            },
            ..default()
        }),
        GameOverText,
    ));
}

