use std::{net::UdpSocket, thread};

use bevy::{asset::Assets, math::Vec3, pbr::{PbrBundle, StandardMaterial}, prelude::{shape, Color, Commands,  Mesh,  Res, ResMut, Transform}, time::{Timer, TimerMode},  utils::default};
use serde_json::from_str;

use crate::{Laser, Message, MyChannel,  ServerDetails, ShootMessage};

pub  fn listen(socket : UdpSocket , channel  :  MyChannel) {
    // Start a thread to listen for messages from the server
    // let socket_clone = global_data.socket.try_clone().expect("Failed to clone socket");
    thread::spawn(move || {
        let mut buf = [0; 1024];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((amt, _src)) => {
                    let msg = String::from_utf8_lossy(&buf[..amt]);
                    if let Err(e) = channel.tx.send(msg.to_string()) {
                        eprintln!("Failed to send message to channel: {}", e);
                    } 
                }
                Err(e) => {
                    eprintln!("Failed to receive from socket: {}", e);
                }
            }
        }
    });
}
pub fn update_ressources(channel : Res<MyChannel> ,  mut globaldata : ResMut<ServerDetails>, 
    mut commands: Commands, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,) {

    let mut rx = channel.rx.lock().unwrap();
    while let Ok(m) = rx.try_recv() {
            let mess : Message = from_str(&m).expect("ERROR");
            if mess.action == "move" {
                let sender_id = mess.senderid.unwrap();
                let new_position = mess.position.unwrap();
                let rotation = mess.rotation.unwrap(); 
                if let Some(players) = &mut globaldata.mess.players {
                    for player in players.iter_mut() {
                        if player.id == sender_id {
                            // println!("Updated position for player {:?}", rotation);
                            player.position = Some(new_position);
                            player.rotation = Some(rotation);
                            break;
                        }
                    }
                }
            }else if mess.action == "shoot" {
                // println!("the player {:?} shoot",  mess.senderid);
                let mess : ShootMessage = from_str(&m).expect("ERROR");
                let  mut player  = None;
                for pl in globaldata.mess.players.clone().unwrap() {
                    if pl.id == mess.senderid {
                        player = Some(pl)
                    }
                }
                if player.is_none() {
                    continue;
                }
                let player = player.unwrap();
                if player.position.is_some() {
                 
                    let avance =  mess.direction * 300.0 * 0.02;

                    // Créer le laser
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Box::new(0.05, 0.05, 10.0))),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(1.0, 0.0, 0.0), // Couleur rouge pour le laser
                                emissive: Color::rgb(1.0, 0.0, 0.0),   // Faire briller le laser
                                ..default()
                            }),
                            transform: Transform::from_translation(mess.origin + avance)
                                .looking_to(mess.direction, Vec3::Y),
                            ..default()
                        
                        },
                        Laser {
                            lifetime: Timer::from_seconds(5.0,TimerMode::Once), // Le laser dure 0.5 secondes
                            hitpoint : mess.hitpoint,
                            origin : mess.origin,
                        },
                    ));

                    if let Some(hit) = mess.hitpoint {
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
                    
                }
            }    
        
    };
}
