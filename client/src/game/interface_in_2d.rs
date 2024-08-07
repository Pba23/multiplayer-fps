pub use bevy::prelude::*;
use crate::game::interface_in_3d::*;
use crate::{game::maze::*, ServerDetails};

// use crate::labyrinths;
// pub use labyrinths::*;
#[derive(Component)]
pub struct RadarBackground;

#[derive(Component)]
pub struct RadarWall;

#[derive(Component)]
pub struct RadarPlayer;

#[derive(Component)]
pub struct RadarOtherPlayer;

pub fn setup_radar(mut commands: Commands, _asset_server: Res<AssetServer>, global_data : Res<ServerDetails>) {
    let radar_size = 200.0;
    let cell_size = radar_size / LABYRINTH_WIDTH as f32;

    // Radar background
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(radar_size), Val::Px(radar_size)),
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Px(20.0), Val::Auto, Val::Px(20.0), Val::Auto),
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.7).into(),
            ..default()
        })
        .insert(RadarBackground);

    // Walls
    let labyrinth = generate_labyrinth(global_data.mess.level.unwrap() as u8);
    for (y, row) in labyrinth.iter().enumerate().rev() {
        for (x, &cell) in row.iter().enumerate().rev() {
            if cell == 1 {
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(cell_size), Val::Px(cell_size)),
                            position_type: PositionType::Absolute,
                            position: UiRect::new(
                                Val::Px(x as f32 * cell_size),
                                Val::Auto,
                                Val::Px(y as f32 * cell_size),
                                Val::Auto,
                            ),
                            ..default()
                        },
                        background_color: Color::rgb(0.8, 0.2, 0.2).into(),
                        ..default()
                    })
                    .insert(RadarWall);
            }
        }
    }

    // Player marker
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(cell_size), Val::Px(cell_size)),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::rgb(0.0, 1.0, 0.0).into(),
            ..default()
        })
        .insert(RadarPlayer);
}
pub fn update_radar(
    player_query: Query<&Transform, With<Player>>,
    mut radar_player_query: Query<&mut Style, With<RadarPlayer>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut radar_player_style) = radar_player_query.get_single_mut() {
            let radar_size = 200.0;
            let cell_size = radar_size / LABYRINTH_WIDTH as f32;

            let x = (player_transform.translation.x / WALL_SIZE).round() * cell_size;
            let y = (-player_transform.translation.z / WALL_SIZE).round() * cell_size;

            radar_player_style.position = UiRect::new(Val::Px(x), Val::Auto, Val::Px(y), Val::Auto);
        }
    }
}