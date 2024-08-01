// use crate::maze;
pub use bevy::prelude::*;
use crate::{game::maze::*, Message, ServerDetails};
use bevy::input::gamepad::{Gamepad, GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonType, GamepadConnectionEvent, GamepadEvent};
pub const WALL_SIZE: f32 = 7.0; // Taille du mur

#[derive(Component)]
pub struct OtherPlayer;

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
    global_data : Res<ServerDetails>
) {
    println!("GLOBAL VARIABLES {:?}" , global_data);
    // Define colors for player, wall, and floor
    let player_color = Color::rgb(0.0, 1.0, 0.0); // Green
    let wall_color = Color::rgb(0.1, 0.1, 0.1); // black
    let floor_color = Color::rgb(0.95, 0.95, 0.95); // Light grey

    // Create materials
    let player_material = materials.add(StandardMaterial {
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
        let (start_x, start_y) = starting_positions[pl.id as usize -1 ];
        
        if pl.id == global_data.mess.clone().curr_player.unwrap().id {
            commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 2. })),
                material: player_material.clone(),
                transform: Transform {
                    translation: Vec3::new(start_x as f32 * WALL_SIZE, 0.5, -(start_y as f32) * WALL_SIZE),
                    ..default()
                },
                ..default()
            })
            .insert(Player);
        } else {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 2. })),
                    material: player_material.clone(),
                    transform: Transform {
                        translation: Vec3::new(start_x as f32 * WALL_SIZE, 0.5, -(start_y as f32) * WALL_SIZE),
                        ..default()
                    },
                    ..default()
                })
                .insert(OtherPlayer);
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
                            translation: Vec3::new(x as f32 * WALL_SIZE, 0.5, -(y as f32) * WALL_SIZE),
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
            transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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
    mut gamepad_evr: EventReader<GamepadEvent>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    mut button_evr: EventReader<GamepadButtonChangedEvent>,
    global_data : Res<ServerDetails>
) {
    let mut direction = Vec3::ZERO;
    let mut rotation = Quat::IDENTITY;
    let mut current_position = Vec3::ZERO;

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
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        rotation *= Quat::from_rotation_y(0.05);
    }
    if keyboard_input.pressed(KeyCode::Right) {
        rotation *= Quat::from_rotation_y(-0.05);
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
        direction.x += x_axis;
    }
    if let Some(y_axis) = axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)) {
        direction.z -= y_axis;
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
        // Deuxième passe : appliquer les changements
        let mut binding = param_set.p0();
        let mut player_transform = binding.single_mut();
        player_transform.translation = new_position;
        player_transform.rotation = rotation;
        
        // send new position to the server
        let mut mes = Message{action : String::from("move") , level : None , players : None , curr_player : None, position: Some(new_position)};
        let json_data = serde_json::to_string(&mes).unwrap();

        global_data.socket.send_to(json_data.as_bytes(), global_data.ip_address.clone());
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
        for mut camera_transform in camera_query.iter_mut() {
            // Positionnez la caméra juste au-dessus de la tête du joueur
            let camera_offset = Vec3::new(0.0, WALL_SIZE/2.0, 0.0); // Ajustez la hauteur (1.5) selon vos besoins
            camera_transform.translation = player_transform.translation + camera_offset;

            // Calculez la direction vers laquelle le joueur regarde
            let forward = player_transform.forward();

            // Positionnez un point de focus légèrement devant le joueur
            let focus_point = player_transform.translation + forward * 10.0; // Le '2.0' détermine la distance du point de focus

            // Faites regarder la caméra vers ce point de focus
            camera_transform.look_at(focus_point, Vec3::Y);
        }
    }
}
