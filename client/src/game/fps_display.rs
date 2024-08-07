#[derive(Component)]
pub struct FpsText;

use bevy::prelude::*;

pub fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 50.0,
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
                text.sections[1].value = format!("{:.2}", 30.0+average);
            }
        }
    }
}