use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{
    netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport},
    renet::{ConnectionConfig, DefaultChannel, RenetClient},
    RenetClientPlugin,
};
use shared::events_set::AddEventSet;
use shared::server_packets::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let server_address = "127.0.0.1:5000".parse().unwrap();
        let client = RenetClient::new(ConnectionConfig::default());
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let authentication = ClientAuthentication::Unsecure {
            protocol_id: 0,
            client_id: current_time.as_micros() as u64,
            server_addr: server_address,
            user_data: None,
        };
        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
        app.add_plugins(RenetClientPlugin)
            .add_plugins(NetcodeClientPlugin)
            .insert_resource(client)
            .insert_resource(transport)
            .add_event_set::<ServerPacketsWriter>()
            .add_systems(PreUpdate, receive_server_packets);
    }
}

fn receive_server_packets(mut client: ResMut<RenetClient>, mut packets: ServerPacketsWriter) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        handle_server_packet(&mut packets, &message);
    }
}
