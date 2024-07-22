use bevy::prelude::*;

pub struct ConnectionScreenPlugin;

impl Plugin for ConnectionScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConnectionState>()
            .add_startup_system(setup_connection_screen)
            .add_system(button_system)
            .add_system(text_input_system);
    }
}

#[derive(Resource, Default)]
struct ConnectionState {
    ip_address: String,
    username: String,
}

#[derive(Component)]
enum ButtonAction {
    Connect,
    SetIp,
    SetUsername,
}

fn setup_connection_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        })
        .with_children(|parent| {
            // IP Address input
            parent.spawn(TextBundle::from_section(
                "Enter IP Address:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
            parent.spawn((
                TextBundle::from_section(
                    "".to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                ButtonAction::SetIp,
            ));

            // Username input
            parent.spawn(TextBundle::from_section(
                "Enter Username:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
            parent.spawn((
                TextBundle::from_section(
                    "".to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                ButtonAction::SetUsername,
            ));

            // Connect button
            spawn_button(
                parent,
                asset_server.load("fonts/FiraSans-Bold.ttf"),
                "Connect",
                ButtonAction::Connect,
            );
        });
}

fn spawn_button(parent: &mut ChildBuilder, font: Handle<Font>, text: &str, action: ButtonAction) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            },
            action,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &ButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut connection_state: ResMut<ConnectionState>,
) {
    for (interaction, action, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.35, 0.75, 0.35).into();
                match action {
                    ButtonAction::Connect => {
                        println!(
                            "Connecting to {} with username {}",
                            connection_state.ip_address, connection_state.username
                        );
                        // Here you can add the logic to initiate the connection to the server
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn text_input_system(
    mut char_input_events: EventReader<ReceivedCharacter>,
    mut query: Query<(&mut Text, &ButtonAction)>,
    mut connection_state: ResMut<ConnectionState>,
) {
    for event in char_input_events.iter() {
        for (mut text, action) in query.iter_mut() {
            match action {
                ButtonAction::SetIp => {
                    connection_state.ip_address.push(event.char);
                    text.sections[0].value = connection_state.ip_address.clone();
                }
                ButtonAction::SetUsername => {
                    connection_state.username.push(event.char);
                    text.sections[0].value = connection_state.username.clone();
                }
                _ => {}
            }
        }
    }
}
