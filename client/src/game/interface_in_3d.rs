// use bevy::asset::LoadState;
// use crate::maze;
use crate::{game::maze::*, Message, ServerDetails};
pub use bevy::gltf::Gltf;
pub use bevy::gltf::GltfMesh;
use bevy::input::gamepad::GamepadEvent;
use bevy::input::mouse::MouseMotion;
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
    // let wall_color = Color::rgb(0.1, 0.1, 0.1); // black
    // let floor_color = Color::rgb(0.95, 0.95, 0.95); // Light grey
    let wall_texture_handle = asset_server.load("wall_texture1.png");
    let floor_texture_handle = asset_server.load("floor_texture.png");

    // Create materials
    let _player_material = materials.add(StandardMaterial {
        base_color: player_color,
        ..Default::default()
    });
    let wall_material = materials.add(StandardMaterial {
        base_color_texture: Some(wall_texture_handle),
        ..Default::default()
    });
    let floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(floor_texture_handle),
        ..Default::default()
    });

    // Setup player entity
    print!("-------------------- level {:?}", global_data.mess.level.clone().unwrap());
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

    for pl in &global_data.mess.players.clone().unwrap() {
        let (start_x, start_y) = starting_positions[pl.id as usize - 1];
        // if let Some(model) = &player_model.0 {
        

        if pl.id == global_data.mess.clone().curr_player.unwrap().id {
            let mut entity = commands.spawn(SceneBundle {
                scene: asset_server.load("armes.glb#Scene0"),
                transform: Transform {
                    translation: Vec3::new(
                        start_x as f32 * WALL_SIZE,
                        2.5,
                        -(start_y as f32) * WALL_SIZE,
                    ), // Augmentez y pour élever le modèle
                    scale: Vec3::splat(0.05), // Ajustez l'échelle si nécessaire
                    ..Default::default()
                },
                ..Default::default()
            });
            entity.insert(Player{});
        } else {
            let mut entity = commands.spawn(SceneBundle {
                scene: asset_server.load("Soldier.glb#Scene0"),
                transform: Transform {
                    translation: Vec3::new(
                        start_x as f32 * WALL_SIZE,
                        0.05,
                        -(start_y as f32) * WALL_SIZE,
                    ), // Augmentez y pour élever le modèle
                    scale: Vec3::splat(0.1), // Ajustez l'échelle si nécessaire
                    ..Default::default()
                },
                ..Default::default()
            });
            entity.insert(OtherPlayer { id: pl.id });
        }

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
            intensity: 30000.0,
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
    mut mouse_motion_events: EventReader<MouseMotion>,
    global_data: Res<ServerDetails>,
) {
    let mut direction = Vec3::ZERO;
    let mut rotation_delta = 0.0;
    let current_position: Vec3;
    let mut current_rotation: Quat;
    // Première passe : lire la position et la rotation du joueur
    {
        let binding = param_set.p0();
        let player_transform = binding.single();
        current_position = player_transform.translation;
        current_rotation = player_transform.rotation;
    }
    // Gestion de la rotation avec la souris
    for event in mouse_motion_events.iter() {
        rotation_delta -= event.delta.x * 0.005; // Ajustez la sensibilité ici
    }
    // Gestion des entrées clavier pour les déplacements latéraux
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Z) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Q) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
    
     // Gestion des entrées gamepad
     let gamepad = Gamepad::new(0);

     if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger)) {
         println!("SHOOT");
     }
 
     // Mouvement avec le stick analogique gauche
     if let Some(x_axis) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)) {
         direction.x += x_axis;
     }
     if let Some(y_axis) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)) {
         direction.z -= y_axis;
     }
 
     // Rotation avec le stick analogique droit
     if let Some(x_axis) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX)) {
        current_rotation *= Quat::from_rotation_y(-x_axis * 0.05);
     }
    // Normaliser la direction pour un mouvement cohérent en diagonale
    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }
    let speed = WALL_SIZE; // Vitesse de déplacement en unités par seconde
    let movement = current_rotation * (direction * speed * time.delta_seconds());
    let new_position = current_position + movement;
    // Appliquer la rotation
    let new_rotation = current_rotation * Quat::from_rotation_y(rotation_delta);
    // Vérifier les collisions
    let wall_query = param_set.p1();
    if !will_collide_with_wall(new_position, &wall_query) {
        let mut binding = param_set.p0();
        let mut player_transform = binding.single_mut();
        if new_position != player_transform.translation || new_rotation != player_transform.rotation {
            // Envoyer la nouvelle position au serveur
            let mes = Message {
                action: String::from("move"),
                level: None,
                players: None,
                curr_player: None,
                position: Some(crate::Vec3::from_v3(
                    new_position.x,
                    current_position.y * 0.05,
                    new_position.z,
                )),
                senderid: Some(global_data.mess.curr_player.clone().unwrap().id),
                rotation: Some(new_rotation),
            };
            let json_data = serde_json::to_string(&mes).unwrap();
            global_data
                .socket
                .send_to(json_data.as_bytes(), global_data.ip_address.clone()).expect("Failed to send message");
        }
        // Deuxième passe : appliquer les changements
        player_transform.translation = new_position;
        player_transform.rotation = new_rotation;
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
    time: Res<Time>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) = (player_query.get_single(), camera_query.get_single_mut()) {
        let camera_offset = Vec3::new(0.04, 0.25, 0.25);
        let target_position = player_transform.translation + player_transform.rotation * camera_offset;
        let target_rotation = player_transform.rotation;

        // Facteur de lissage (ajustez selon vos préférences)
        let smoothness = 15.0;
        let delta_time = time.delta_seconds();

        // Interpolation linéaire de la position
        camera_transform.translation = camera_transform.translation.lerp(target_position, smoothness * delta_time);

        // Interpolation sphérique de la rotation
        camera_transform.rotation = camera_transform.rotation.slerp(target_rotation, smoothness * delta_time);
    }
}


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
