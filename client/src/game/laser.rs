pub use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::game::cylinder;
use crate::{game::interface_in_3d::*, Message, ServerDetails , game::vector3d::*};

use super::cylinder::Object;

#[derive(Component , Clone)]
pub struct Laser {
    pub origin : Vec3,
    pub lifetime: Timer,
    pub hitpoint : Option<hit_info>
}
#[derive(Debug , Clone, Copy , Serialize, Deserialize)]
pub struct hit_info {
    point : Vec3,
    playerid : u32
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
    pub senderid : u32,
    pub hitpoint : Option<hit_info>
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
        let avance =  ray_direction * 300.0 * 0.02  + point_a_droite( player_transform.forward().normalize());
        
        let laser = Laser {
            origin : player_transform.translation ,
            lifetime : Timer::from_seconds(5.0,TimerMode::Once),
            hitpoint : intersect_cylinder(cylinder::Ray{origin : Vector3D::from_v3(player_transform.translation) , direction :  Vector3D::from_v3(ray_direction)} , globaldata.mess.players.clone().unwrap() ) 
        };
        // Créer le laser
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.05, 0.05, 10.0))),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.8, 0.6, 0.2), // Couleur rouge pour le laser
                    emissive: Color::rgb(1.0, 0.0, 0.0),   // Faire briller le laser
                    ..default()
                }),
                transform: Transform::from_translation(laser.origin + avance)
                    .looking_to(ray_direction, Vec3::Y),
                ..default()
                
            }, 
            laser.clone(),
        ));

        let  mes = ShootMessage{action : String::from("shoot") , origin : laser.origin  , senderid : globaldata.mess.curr_player.clone().unwrap().id , direction : ray_direction  , hitpoint : laser.hitpoint};
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
            // println!("next pos {}" , transform.translation);
        }
    }
}
pub fn check_laser_collisions(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &Laser)>,
    // player_query: Query<(Entity, &Transform), With<Wall>>,
    walls :  Query<&Transform, With<Wall>>,
    mut globaldata : ResMut<ServerDetails>, 
) {
    for (laser_entity, laser_transform, laser) in laser_query.iter() {
        if laser.lifetime.finished() || will_collide_with_wall(laser_transform.translation, &walls) {
            commands.entity(laser_entity).despawn();
        } else if laser.hitpoint.is_some() {
            if laser.origin.distance(laser_transform.translation) > laser.origin.distance(laser.hitpoint.unwrap().point)  {
                commands.entity(laser_entity).despawn();

                println!("HIT THE PLAYER {} " , laser.hitpoint.unwrap().playerid);

                if let Some(players) = &mut globaldata.mess.players {
                    for player in players.iter_mut() {
                        if player.id == laser.hitpoint.unwrap().playerid && player.lives > 0{
                            // println!("Updated position for player {:?}", rotation);
                            player.lives -= 1;
                            break;
                        }
                    }
                }
            }
        }
        // else {
        //     for (player_entity, player_transform) in player_query.iter() {
        //         // println!("playerddd djdjdjd");
        //         if (player_transform.translation - laser_transform.translation).length() < 1.0 {
        //             println!("Player hit by laser!");
        //             commands.entity(player_entity).despawn();
        //             commands.entity(laser_entity).despawn();
        //             break;
        //         }
        //     }
        // }
    }
}


fn intersect_cylinder(ray: cylinder::Ray , players: Vec<crate::Player>) -> Option<hit_info> {
    let mut res = Vec::new();
    for p in players {
        if let  Some(position) = p.position {
            let cylinder = cylinder::Cylinder::new(Vector3D::from_v3b(position), Vector3D::new(0.0, 1.0, 0.0), 0.5, 5.5);
            
            if let Some(a) = cylinder.intersect(&ray) {
               res.push((p.id ,Vector3D::to_v3(a.point)))
            }

        }
    }
    if res.is_empty() {
       return  None
    }
    let mut  p = res[0];
    //  find the intersection most clear to the lazer origin
    for (id , point) in res {
        if point.distance(Vector3D::to_v3(ray.origin)) <  p.1.distance(Vector3D::to_v3(ray.origin)) {
            p = (id , point)
        }
    }
    Some(hit_info { point: p.1, playerid: p.0 })
}

fn point_a_droite(forward: Vec3) -> Vec3 {
    let distance  = 0.1;
    let up = Vec3::Y;
    let right = forward.cross(up).normalize();
    // Calcul du nouveau point à la distance donnée à droite
    right * distance
}
pub fn delete_dead_players(mut commands: Commands , curr_player : Query<(Entity , &mut Transform), With<Player>>,
    mut player_query: Query<(Entity  , &OtherPlayer), With<OtherPlayer>>,
    mut globaldata: ResMut<ServerDetails>) {

        if let Some(players) = &globaldata.mess.players {
            println!("find player {}", player_query.iter_mut().count());

            for (entity ,   player) in player_query.iter_mut() {
                for global_player in players {
                    if global_player.id == player.id {
                        if global_player.lives == 0 {
                            // commands.entity(entity).despawn();
                            commands.entity(entity).despawn_recursive()
                        }
                    } 
                    
                }
            }
            println!("COUNT {}", player_query.iter_mut().count());
        }

}