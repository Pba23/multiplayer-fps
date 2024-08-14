pub use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::game::cylinder;
use crate::FpsText;
use crate::{game::interface_in_3d::*, ServerDetails , game::vector3d::*};

use super::cylinder::Object;



#[derive(Component , Clone)]
pub struct Laser {
    pub origin : Vec3,
    pub lifetime: Timer,
    pub hitpoint : Option<HitInfo>
}
#[derive(Debug , Clone, Copy , Serialize, Deserialize)]
pub struct HitInfo {
    pub point : Vec3,
    pub playerid : u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  ShootMessage {
    pub action : String,
    pub origin : Vec3,
    pub direction : Vec3,
    pub senderid : u32,
    pub hitpoint : Option<HitInfo>
}
const INFO: f64 = 40.0;


pub fn player_shoot(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    player_query: Query<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    buttons: Res<Input<GamepadButton>>,
    mut globaldata : ResMut<ServerDetails>

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
            hitpoint : intersect_cylinder(cylinder::Ray{origin : Vector3D::from_v3(player_transform.translation) , direction :  Vector3D::from_v3(ray_direction)} , globaldata.mess.players.clone() ) 
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

        // lose life
        if let Some(hit) = laser.hitpoint {
            if let Some(players) = &mut globaldata.mess.players {
                for player in players.iter_mut() {
                    if player.id == hit.playerid && player.lives > 0{
                        // println!("Updated position for player {:?}", rotation);
                        player.lives -= 1;
                        break;
                    }
                }
            }
        }

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
    walls :  Query<&Transform, With<Wall>>,
) {
    for (laser_entity, laser_transform, laser) in laser_query.iter() {
        if laser.lifetime.finished() || will_collide_with_wall(laser_transform.translation, &walls) {
            commands.entity(laser_entity).despawn();
        } else if laser.hitpoint.is_some() {
            if laser.origin.distance(laser_transform.translation) > laser.origin.distance(laser.hitpoint.unwrap().point)  {
                commands.entity(laser_entity).despawn();

                println!("HIT THE PLAYER {} " , laser.hitpoint.unwrap().playerid);

                // if let Some(players) = &mut globaldata.mess.players {
                //     for player in players.iter_mut() {
                //         if player.id == laser.hitpoint.unwrap().playerid && player.lives > 0{
                //             // println!("Updated position for player {:?}", rotation);
                //             player.lives -= 1;
                //             break;
                //         }
                //     }
                // }
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


fn intersect_cylinder(ray: cylinder::Ray , players: Option<Vec<crate::Player>>) -> Option<HitInfo> {
    if players.is_none() {
        return None
    }
    let players = players.unwrap();
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
    Some(HitInfo { point: p.1, playerid: p.0 })
}

fn point_a_droite(forward: Vec3) -> Vec3 {
    let distance  = 0.1;
    let up = Vec3::Y;
    let right = forward.cross(up).normalize();
    // Calcul du nouveau point à la distance donnée à droite
    right * distance
}
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
pub fn shoot(mut query: Query<&mut Text, With<FpsText>>, diagnostics: Res<Diagnostics>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            if let Ok(mut text) = query.get_single_mut() {
                text.sections[1].value = format!("{:.2}", INFO+average);
            }
        }
    }
}
pub fn delete_dead_players(mut commands: Commands ,
    mut player_query: Query<(Entity  , &OtherPlayer), With<OtherPlayer>>,
    mut globaldata: ResMut<ServerDetails>) {

        if let Some(players) = &globaldata.mess.players {
            // println!("find player {}", player_query.iter_mut().count());
            // let (currentity , c)  = curr_player.get_single().unwrap();
            // let currid = globaldata.mess.curr_player.clone().unwrap().id;
            for (entity ,   player) in player_query.iter_mut() {
                for global_player in players {
                    if global_player.id == player.id {
                        if global_player.lives == 0 {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                }
            }
            let mut  c = players.clone();
            c.retain(|p|  p.lives > 0);
            if c.len() == 0 {
                globaldata.mess.players = None;
            } else {
                globaldata.mess.players = Some(c);
            }
        }

}