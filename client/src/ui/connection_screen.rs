use bevy::prelude::*;
use bevy::ui::widget::Button;
use tokio::sync::mpsc;
use crate::connect_to_server;

pub struct ConnectionScreenPlugin;

impl Plugin for ConnectionScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConnectionState>()
           .add_startup_system(setup_connection_screen)
           .add_system(button_system)
           .add_system(handle_server_messages)
           .add_system(input_system);
    }
}

#[derive(Resource, Default)]
struct ConnectionState {
    ip_address: String,
    username: String,
    sender: Option<mpsc::Sender<String>>,
    receiver: Option<mpsc::Receiver<String>>,
}

#[derive(Component)]
enum ButtonAction {
    Connect,
    SetIp,
    SetUsername,
}

#[derive(Component)]
struct IpAddressInput;

#[derive(Component)]
struct UsernameInput;

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
            parent.spawn((TextBundle::from_section(
                "127.0.0.1",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ), IpAddressInput));

            // Username input
            parent.spawn(TextBundle::from_section(
                "Enter Username:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
            parent.spawn((TextBundle::from_section(
                "Player1",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ), UsernameInput));

            // Connect button
            spawn_button(parent, asset_server.load("fonts/FiraSans-Bold.ttf"), "Connect", ButtonAction::Connect);
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

fn input_system(
    mut interaction_query: Query<(&Interaction, &ButtonAction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<IpAddressInput>>,
        Query<&mut Text, With<UsernameInput>>
    )>,
    mut connection_state: ResMut<ConnectionState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (interaction, action, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.35, 0.75, 0.35).into();
                match action {
                    ButtonAction::Connect => {
                        println!("Connecting to {} with username {}", connection_state.ip_address, connection_state.username);
                        let (tx, rx) = mpsc::channel(32);
                        connection_state.sender = Some(tx.clone());
                        connection_state.receiver = Some(rx);
                        let ip = connection_state.ip_address.clone();
                        let username = connection_state.username.clone();
                        tokio::spawn(async move {
                            connect_to_server(ip, username, tx).await;
                        });
                    },
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

    for key in keyboard_input.get_just_pressed() {
        if let Some(mut ip_text) = param_set.p0().iter_mut().next() {
            if *key == KeyCode::Return {
                connection_state.ip_address = ip_text.sections[0].value.clone();
                println!("IP set to: {}", connection_state.ip_address);
            } else if *key != KeyCode::Back {
                ip_text.sections[0].value.push(format!("{:?}", key).chars().next().unwrap());
            }
        }
        if let Some(mut username_text) = param_set.p1().iter_mut().next() {
            if *key == KeyCode::Return {
                connection_state.username = username_text.sections[0].value.clone();
                println!("Username set to: {}", connection_state.username);
            } else if *key != KeyCode::Back {
                username_text.sections[0].value.push(format!("{:?}", key).chars().next().unwrap());
            }
        }
    }
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &ButtonAction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut connection_state: ResMut<ConnectionState>,
) {
    for (interaction, action, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = Color::rgb(0.35, 0.75, 0.35).into();
                match action {
                    ButtonAction::Connect => {
                        println!("Connecting to {} with username {}", connection_state.ip_address, connection_state.username);
                        let (tx, rx) = mpsc::channel(32);
                        connection_state.sender = Some(tx.clone());
                        connection_state.receiver = Some(rx);
                        let ip = connection_state.ip_address.clone();
                        let username = connection_state.username.clone();
                        tokio::spawn(async move {
                            connect_to_server(ip, username, tx).await;
                        });
                    },
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

fn handle_server_messages(
    mut connection_state: ResMut<ConnectionState>,
) {
    if let Some(receiver) = &mut connection_state.receiver {
        while let Ok(message) = receiver.try_recv() {
            println!("Message from server: {}", message);
            // Process the message from the server, such as updating the UI or game state
        }
    }
}
