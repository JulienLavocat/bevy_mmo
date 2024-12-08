use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{
    netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent},
    RenetServerPlugin,
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let server = RenetServer::new(ConnectionConfig::default());
        let server_address_raw = "0.0.0.0:5000";
        let server_address = server_address_raw.parse().unwrap();
        let socket = UdpSocket::bind(server_address).unwrap();
        let server_config = ServerConfig {
            current_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
            max_clients: 1024,
            protocol_id: 0,
            public_addresses: vec![server_address],
            authentication: ServerAuthentication::Unsecure,
        };
        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

        app.add_plugins(RenetServerPlugin)
            .insert_resource(server)
            .add_plugins(NetcodeServerPlugin)
            .insert_resource(transport)
            .add_systems(Update, (handle_events, receive_messages));
        info!("server listening on {:?}", server_address_raw);
    }
}

fn handle_events(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("{} connected", client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("{} disconnected for: {}", client_id, reason);
            }
        }
    }
}

fn receive_messages(mut server: ResMut<RenetServer>) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            println!("Got message: {:?}", message)
        }
    }
}
