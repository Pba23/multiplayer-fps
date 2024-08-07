pub use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{game::interface_in_3d::*, Message, ServerDetails};

#[derive(Component)]
pub struct Laser {
    pub lifetime: Timer,
}
#[derive(Component)]
struct PlayerInfo {
    pub id: u32,
    // Ajoutez d'autres champs nécessaires ici
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  ShootMessage {
    pub action : String,
    pub origin : Vec3,
    pub direction : Vec3,
    pub senderid : u32
}


pub fn player_shoot(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    gamepad_button: Res<Input<GamepadButton>>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    buttons: Res<Input<GamepadButton>>,
    globaldata : Res<ServerDetails>

) {
    let player_transform= player_query.single();

    let gamepad = Gamepad::new(0);
    
    if mouse_button_input.just_pressed(MouseButton::Left) || 
        buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger)) {
        let ray_direction = player_transform.forward();
        
        // Créer le laser
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.05, 0.05, 80.0))),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(1.0, 0.0, 0.0), // Couleur rouge pour le laser
                    emissive: Color::rgb(1.0, 0.0, 0.0),   // Faire briller le laser
                    ..default()
                }),
                transform: Transform::from_translation(player_transform.translation)
                    .looking_to(ray_direction, Vec3::Y),
                ..default()
            }, 
            Laser {
                lifetime: Timer::from_seconds(0.5,TimerMode::Once), // Le laser dure 0.5 secondes
            },
        ));
        let  mes = ShootMessage{action : String::from("shoot") , origin : player_transform.translation  , senderid : globaldata.mess.curr_player.clone().unwrap().id , direction : -ray_direction};
        let json_data = serde_json::to_string(&mes).unwrap();
        globaldata.socket.send_to(json_data.as_bytes(), globaldata.ip_address.clone()).expect("failed to send shoot");
    }

}
pub fn update_laser_positions(
    time: Res<Time>,
    mut laser_query: Query<(&mut Transform, &mut Laser)>,
) {
    for (mut transform, mut laser) in laser_query.iter_mut() {
        laser.lifetime.tick(time.delta());
        if !laser.lifetime.finished() {
            let forward = transform.forward();
            transform.translation += forward * 300.0 * time.delta_seconds();
        }
    }
}
pub fn check_laser_collisions(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &Laser)>,
    player_query: Query<(Entity, &Transform), With<Wall>>,
) {
    for (laser_entity, laser_transform, laser) in laser_query.iter() {
        if laser.lifetime.finished() {
            commands.entity(laser_entity).despawn();
        } else {
            for (player_entity, player_transform) in player_query.iter() {
                println!("playerddd djdjdjd");
                if (player_transform.translation - laser_transform.translation).length() < 1.0 {
                    println!("Player hit by laser!");
                    commands.entity(player_entity).despawn();
                    commands.entity(laser_entity).despawn();
                    break;
                }
            }
        }
    }
}
