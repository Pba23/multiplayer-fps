// use bevy::asset::LoadState;
// use crate::maze;
use crate::{game::maze::*, Message, ServerDetails};
pub use bevy::gltf::Gltf;
pub use bevy::gltf::GltfMesh;
use bevy::input::gamepad::{GamepadButtonChangedEvent, GamepadEvent};
pub use bevy::prelude::*;
pub const WALL_SIZE: f32 = 7.0; // Taille du mur

#[derive(Component)]
pub struct OtherPlayer {
    pub id: u32,
}

#[derive(Component)]

pub struct Player;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct MainCamera;
pub const LABYRINTH_WIDTH: usize = 20;
pub const LABYRINTH_HEIGHT: usize = 20;
// pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let player_model = asset_server.load("assets/Soldier.glb");
//     commands.insert_resource(PlayerModel(Some(player_model)));
// }

// #[derive(Resource, Default)]
// pub struct PlayerModel(Option<Handle<Scene>>);
// pub fn check_model_loaded(player_model: Res<PlayerModel>, asset_server: Res<AssetServer>) {
//     if let Some(handle) = &player_model.0 {
//         match asset_server.get_load_state(handle) {
//             LoadState::Loading => println!("Le modèle est en cours de chargement..."),
//             LoadState::Loaded => println!("Le modèle est chargé avec succès!"),
//             LoadState::Failed => println!("Échec du chargement du modèle!"),
//             LoadState::Unloaded => println!("Le modèle n'est pas chargé."),
//             _ => println!("autreeeee"),
//         }
//     } else {
//         println!("Aucun modèle n'est assigné.");
//     }
// }
// pub fn debug_scene_entities(query: Query<Entity, With<Handle<Scene>>>) {
//     for entity in query.iter() {
//         println!("Entité avec SceneBundle trouvée: {:?}", entity);
//     }
// }

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    global_data: Res<ServerDetails>,
    // player_model: Res<PlayerModel>,
    asset_server: Res<AssetServer>,
) {
    println!("GLOBAL VARIABLES {:?}", global_data);
    // Define colors for player, wall, and floor
    let player_color = Color::rgb(0.0, 1.0, 0.0); // Green
    let wall_color = Color::rgb(0.1, 0.1, 0.1); // black
    let floor_color = Color::rgb(0.95, 0.95, 0.95); // Light grey

    // Create materials
    let _player_material = materials.add(StandardMaterial {
        base_color: player_color,
        ..Default::default()
    });
    let wall_material = materials.add(StandardMaterial {
        base_color: wall_color,
        ..Default::default()
    });
    let floor_material = materials.add(StandardMaterial {
        base_color: floor_color,
        ..Default::default()
    });

    // Setup player entity
    let labyrinth = generate_labyrinth(global_data.mess.level.unwrap() as u8);
    // Find starting positions (positions with value 2)
    let mut starting_positions = Vec::new();
    for (y, row) in labyrinth.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 2 {
                starting_positions.push((x, y));
            }
        }
    }

    // Choose a random starting position
    // use rand::seq::SliceRandom;
    // let mut rng = rand::thread_rng();

    // let (start_x, start_y) = starting_positions[global_data.mess.curr_player.clone().unwrap().id as usize -1 ];
    // Setup player entity at the chosen starting position
    for pl in &global_data.mess.players.clone().unwrap() {
        let (start_x, start_y) = starting_positions[pl.id as usize - 1];
        // if let Some(model) = &player_model.0 {
        let mut entity = commands.spawn(SceneBundle {
            scene: asset_server.load("Soldier.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(
                    start_x as f32 * WALL_SIZE,
                    0.5,
                    -(start_y as f32) * WALL_SIZE,
                ), // Augmentez y pour élever le modèle
                scale: Vec3::splat(0.1), // Ajustez l'échelle si nécessaire
                ..Default::default()
            },
            ..Default::default()
        });

        if pl.id == global_data.mess.clone().curr_player.unwrap().id {
            entity.insert(Player);
        } else {
            entity.insert(OtherPlayer { id: pl.id });
        }
        // } else {
        //     // Fallback to a cube if the model isn't loaded yet
        //     let mut entity = commands.spawn(PbrBundle {
        //         mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        //         material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        //         transform: Transform {
        //             translation: Vec3::new(
        //                 start_x as f32 * WALL_SIZE,
        //                 0.5,
        //                 -(start_y as f32) * WALL_SIZE,
        //             ),
        //             ..Default::default()
        //         },
        //         ..Default::default()
        //     });

        //     if pl.id == global_data.mess.clone().curr_player.unwrap().id {
        //         entity.insert(Player);
        //     } else {
        //         entity.insert(OtherPlayer);
        //     }
        // }
    }

    // Create entities for the labyrinth
    for y in 0..LABYRINTH_HEIGHT {
        for x in 0..LABYRINTH_WIDTH {
            if labyrinth[y][x] == 1 {
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: WALL_SIZE })),
                        material: wall_material.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                x as f32 * WALL_SIZE,
                                0.5,
                                -(y as f32) * WALL_SIZE,
                            ),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Wall);
            } else {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane {
                        size: WALL_SIZE,
                        subdivisions: 0,
                    })),
                    material: floor_material.clone(),
                    transform: Transform {
                        translation: Vec3::new(x as f32 * WALL_SIZE, 0.0, -(y as f32) * WALL_SIZE),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }

    // Setup 3D camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MainCamera);

    // Add a light source
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut param_set: ParamSet<(
        Query<&mut Transform, With<Player>>,
        Query<&Transform, With<Wall>>,
    )>,
    _gamepad_evr: EventReader<GamepadEvent>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    mut button_evr: EventReader<GamepadButtonChangedEvent>,
    global_data: Res<ServerDetails>,
) {
    let mut direction = Vec3::ZERO;
    let mut rotation: Quat;
    let current_position: Vec3;

    // Première passe : lire la position et la rotation du joueur
    {
        let binding = param_set.p0();
        let player_transform = binding.single();
        current_position = player_transform.translation;
        rotation = player_transform.rotation;
    }

    // Gestion des entrées clavier
    if keyboard_input.pressed(KeyCode::Up) {
        println!("keyup");
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        rotation *= Quat::from_rotation_y(0.025);
    }
    if keyboard_input.pressed(KeyCode::Right) {
        rotation *= Quat::from_rotation_y(-0.025);
    }

    // Gestion des entrées gamepad
    let gamepad = Gamepad::new(0);

    if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger)) {
        println!("SHOOT");
    }
    for button_event in button_evr.iter() {
        if button_event.value == 1.0 {
            println!("Button pressed: {:?}", button_event.button_type);
        }
    }

    // Mouvement avec le stick analogique gauche
    if let Some(x_axis) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)) {
        direction.x -= x_axis;
    }
    if let Some(y_axis) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)) {
        direction.z += y_axis;
    }

    // Rotation avec le stick analogique droit
    if let Some(x_axis) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX)) {
        rotation *= Quat::from_rotation_y(-x_axis * 0.05);
    }

    // Normaliser la direction pour un mouvement cohérent en diagonale
    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    let speed = WALL_SIZE; // Vitesse de déplacement en unités par seconde
    let movement = rotation * (direction * speed * time.delta_seconds());
    let new_position = current_position + movement;

    // Vérifier les collisions
    let wall_query = param_set.p1();
    if !will_collide_with_wall(new_position, &wall_query) {
        // println!("the new position {:?}" , new_position);
        let mut binding = param_set.p0();
        let mut player_transform = binding.single_mut();

        if new_position != player_transform.translation || rotation != player_transform.rotation {
            // send new position to the server
            let mut mes = Message {
                action: String::from("move"),
                level: None,
                players: None,
                curr_player: None,
                position: Some(crate::Vec3::fromV3(
                    current_position.x,
                    current_position.y,
                    current_position.z,
                )),
                senderid: Some(global_data.mess.curr_player.clone().unwrap().id),
                rotation: Some(rotation),
            };
            let json_data = serde_json::to_string(&mes).unwrap();

            global_data
                .socket
                .send_to(json_data.as_bytes(), global_data.ip_address.clone());
        }

        // Deuxième passe : appliquer les changements

        player_transform.translation = new_position;
        player_transform.rotation = rotation;
    }
}
pub fn will_collide_with_wall(
    new_position: Vec3,
    wall_query: &Query<&Transform, With<Wall>>,
) -> bool {
    const PLAYER_SIZE: f32 = 1.0; // Taille du joueur
    const COLLISION_THRESHOLD: f32 = (PLAYER_SIZE + WALL_SIZE) / 2.0;

    for wall_transform in wall_query.iter() {
        let wall_pos = wall_transform.translation;

        // Vérifier la collision sur les axes X et Z
        if (new_position.x - wall_pos.x).abs() < COLLISION_THRESHOLD
            && (new_position.z - wall_pos.z).abs() < COLLISION_THRESHOLD
        {
            print!("----------------\n---------------collide here\n");
            return true;
        }
    }
    false
}
pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        // for mut camera_transform in camera_query.iter_mut() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            // Positionnez la caméra juste au-dessus de la tête du joueur
            let camera_offset = Vec3::new(0.0, WALL_SIZE / 2.0, 0.0); // Ajustez la hauteur (1.5) selon vos besoins
            camera_transform.translation = player_transform.translation + camera_offset;

            // Calculez la direction vers laquelle le joueur regarde
            let forward = player_transform.forward();

            // Positionnez un point de focus légèrement devant le joueur
            let focus_point = player_transform.translation - forward * 10.0; // Le '2.0' détermine la distance du point de focus

            // Faites regarder la caméra vers ce point de focus
            camera_transform.look_at(focus_point, Vec3::Y);
        }
    }
}
// pub fn camera_follow_player(
//     player_query: Query<&Transform, With<Player>>,
//     mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
// ) {
//     if let Ok(player_transform) = player_query.get_single() {
//         for mut camera_transform in camera_query.iter_mut() {
//             // Define the radius of the circular path and height offset
            // let radius: f32 = 5.0;
            // let height_offset: f32 = WALL_SIZE / 2.0;

            // // Calculate the angle based on the player's rotation
            // let yaw = player_transform.rotation.to_euler(EulerRot::YXZ).0;

            // // Calculate the new camera position in a circle around the player
            // let x = player_transform.translation.x + radius * yaw.cos();
            // let z = player_transform.translation.z + radius * yaw.sin();
            // let y = player_transform.translation.y + height_offset;

            // // Update the camera's position
            // camera_transform.translation = Vec3::new(x, y, z);

            // // Calculate the forward direction based on the player's rotation
            // let forward = player_transform.forward();

            // // Calculate the focus point slightly in front of the player
            // let focus_distance = 10.0;
            // let focus_point = player_transform.translation + forward * focus_distance;

            // // Make the camera look at the focus point through the player
            // camera_transform.look_at(focus_point, Vec3::Y);
//         }
//     }
// }

pub fn setup_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Percent(50.0),
                    left: Val::Percent(50.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                    ..default()
                },
                image: UiImage::new(asset_server.load("crosshair.png")),
                ..default()
            });
        });
}

pub fn update_position(
    mut player_query: Query<(&mut Transform, &OtherPlayer), With<OtherPlayer>>,
    global_data: Res<ServerDetails>,
) {
    if let Some(players) = &global_data.mess.players {
        for (mut tr, player) in player_query.iter_mut() {
            for global_player in players {
                if global_player.id == player.id {
                    if let Some(new_position) = &global_player.position {
                        tr.translation = Vec3::new(new_position.x, new_position.y, new_position.z); //new_position;
                        tr.rotation = global_player.rotation.unwrap();
                    }
                }
            }
        }
    }
}
