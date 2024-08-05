use std::{borrow::BorrowMut, net::UdpSocket, thread, time::Duration};

use bevy::prelude::{Res, ResMut};
use serde_json::from_str;

use crate::{Message, MyChannel, RadarOtherPlayer, ServerDetails};

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
                    } else {
                        println!("socket it {}" , msg.to_string())
                    }
                }
                Err(e) => {
                    eprintln!("Failed to receive from socket: {}", e);
                }
            }
        }
    });
}
pub fn update_ressources(channel : Res<MyChannel> ,  mut globaldata : ResMut<ServerDetails>) {

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
            }            
        
    };
}