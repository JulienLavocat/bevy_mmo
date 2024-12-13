use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{
    netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig},
    renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent},
    RenetServerPlugin,
};
use shared::{
    client_packets::{handle_client_packet, ClientPacketEventsWriter},
    events_set::AddEventSet,
    server_packets::*,
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
            .add_event_set::<ClientPacketEventsWriter>()
            .add_systems(PreUpdate, (handle_events, receive_client_packets));
        info!("server listening on {:?}", server_address_raw);
    }
}

fn handle_events(mut server_events: EventReader<ServerEvent>, mut server: ResMut<RenetServer>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("{} connected", client_id);
                let packet = &ServerPacket::SpawnPlayer(SpawnPlayer {
                    location: Vec3::new(10.0, 10.0, 10.0),
                });
                server.send_message(
                    *client_id,
                    DefaultChannel::ReliableOrdered,
                    bincode::serialize(packet).unwrap(),
                );
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("{} disconnected for: {}", client_id, reason);
            }
        }
    }
}

fn receive_client_packets(mut server: ResMut<RenetServer>, mut packets: ClientPacketEventsWriter) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            handle_client_packet(&mut packets, &message);
        }
    }
}
