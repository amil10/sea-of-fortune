mod data;
mod level;
mod network;

use bevy::prelude::*;
use level::components::*;
use serde::Serialize;
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::*;

use crate::level::systems::*;
use crate::network::components::*;
use crate::network::systems::*;

pub fn create_env<T: Serialize>(message: String, object: T) -> String {
    let packet: Packet<T> = Packet { payload: object };

    serde_json::to_string(&Envelope {
        message: message,
        packet: serde_json::to_string(&packet).unwrap(),
    })
    .unwrap()
}

fn main() {
    println!("Starting Server");

    // Creating UDP socket connecion

    // Creating ocean level
    let ocean_map = OceanMap { map: build_ocean() };

    println!("Ocean size: {}", ocean_map.map.len());

    let tcpconnections = TcpConnections {
        streams: Vec::new(),
    };

    let connections = TcpResource {
        streams: Arc::new(Mutex::new(tcpconnections)),
    };

    let result = UdpSocket::bind("127.0.0.1:5000");

    if result.is_ok() {
        let udp_socket = result.unwrap();

        println!(
            "UDP Socket listening to {}",
            udp_socket.local_addr().unwrap()
        );

        App::new()
            .insert_resource(connections)
            .insert_resource(ocean_map)
            .insert_resource(Counter::init())
            .insert_resource(Players::init())
            .insert_resource(Enemies::init())
            .insert_resource(UDP { socket: udp_socket })
            .add_systems(Startup, server_start)
            .add_systems(Update, listen)
            .run();
    } else {
        println!("UDP Socket unsuccessfully bound");
        //3 sec cooldown between attempts
        std::thread::sleep(Duration::new(3, 0));
    }

    //.add_systems(Update);
}

pub fn server_start(ocean: Res<OceanMap>, mut players: ResMut<Players>, udp: Res<UDP>) {
    loop {
        let mut buf = [0; 1024];
        let result = udp.socket.recv_from(&mut buf);

        match result {
            Ok((bytes, src)) => {
                let env: Envelope = serde_json::from_slice(&buf[..bytes]).unwrap();

                if env.message.eq("new_player") {
                    let packet: Packet<Player> = serde_json::from_str(&env.packet).unwrap();
                    let mut new_player = packet.payload;

                    println!("Player join request from [{}]", new_player.clone().addr);

                    let mut index = 0;
                    let mut full = true;

                    for mut player in players.player_array.iter() {
                        if !player.used {
                            new_player.id = index;
                            new_player.used = true;
                            players.player_array[index as usize] = new_player.clone();
                            full = false;
                            break;
                        }
                        index += 1;
                    }

                    if full {
                        udp.socket
                            .send_to(
                                create_env(
                                    "full_lobby".to_string(),
                                    "Lobby is full, cannot join right now. Try again later!"
                                        .to_string(),
                                )
                                .as_bytes(),
                                new_player.addr,
                            )
                            .expect("Failed to send [full_lobby] packet");
                    } else {
                        //If lobby isn't full
                        udp.socket
                            .send_to(
                                create_env("joined_lobby".to_string(), new_player.id).as_bytes(),
                                "127.0.0.1:4000",
                            )
                            .expect("Failed to send [id] response packet");

                        println!("Sending ocean overworld...");
                        let mut size = 0;
                        for tile in ocean.map.iter() {
                            size += 1;

                            let expect_msg = "Failed to send ocean tile packet #".to_string()
                                + &size.to_string();

                            udp.socket
                                .send_to(
                                    create_env("load_ocean".to_string(), tile.clone()).as_bytes(),
                                    "127.0.0.1:4000",
                                )
                                .expect(&expect_msg);
                        }
                        println!("Done. Total ocean packets sent: {}", size);
                    }

                    break;
                } else {
                    println!("Recieved invalid packet from [{}]", src.ip())
                }
            }
            Err(e) => {
                eprintln!("Something happened: {}", e);
            }
        }
    }

    //start_tcp_server(&connections);
}

pub fn listen(ocean: Res<OceanMap>, mut players: ResMut<Players>, udp: Res<UDP>) {
    let mut buf = [0; 1024];

    let result = udp.socket.recv_from(&mut buf);

    match result {
        Ok((bytes, src)) => {
            let env: Envelope = serde_json::from_slice(&buf[..bytes]).unwrap();

            if env.message.eq("new_player") {
                let packet: Packet<Player> = serde_json::from_str(&env.packet).unwrap();
                let mut new_player = packet.payload;

                println!("Player join request from [{}]", new_player.addr);

                let mut index = 0;
                let mut full = true;

                for player in players.player_array.iter() {
                    if !player.used {
                        new_player.id = index;
                        new_player.used = true;
                        players.player_array[index as usize] = new_player.clone();
                        full = false;
                        break;
                    }
                    index += 1;
                }

                if full {
                    udp.socket
                        .send_to(
                            create_env(
                                "full_lobby".to_string(),
                                "Lobby is full, cannot join right now. Try again later!"
                                    .to_string(),
                            )
                            .as_bytes(),
                            new_player.addr,
                        )
                        .expect("Failed to send [full_lobby] packet");
                } else {
                    //If lobby isn't full
                    udp.socket
                        .send_to(
                            create_env("joined_lobby".to_string(), new_player.id).as_bytes(),
                            "127.0.0.1:4000",
                        )
                        .expect("Failed to send [id] response packet");

                    println!("Sending ocean overworld...");
                    let mut size = 0;
                    for tile in ocean.map.iter() {
                        size += 1;

                        let expect_msg =
                            "Failed to send ocean tile packet #".to_string() + &size.to_string();

                        udp.socket
                            .send_to(
                                create_env("load_ocean".to_string(), tile.clone()).as_bytes(),
                                "127.0.0.1:4000",
                            )
                            .expect(&expect_msg);
                    }
                    println!("Done. Total ocean packets sent: {}", size);
                }
            } else if env.message.eq("player_leave") {
                let packet: Packet<Player> = serde_json::from_str(&env.packet).unwrap();

                let player = packet.payload;
                let id = player.id;
                let addr = player.addr;

                players.player_array[id as usize].used = false;

                udp.socket
                    .send_to(
                        create_env("leave_success".to_string(), "null".to_string()).as_bytes(),
                        addr.clone(),
                    )
                    .expect("Failed to send [leave_success] packet");
            } else {
                println!(
                    "Recieved invalid packet from [{}]: {}",
                    src.ip(),
                    env.message
                );
            }
        }
        Err(e) => {
            eprintln!("Something happened: {}", e);
        }
    }
}
