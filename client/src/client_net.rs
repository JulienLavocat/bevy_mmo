use std::{net::UdpSocket, time::SystemTime};

use bevy::{
    app::{Plugin, Startup, Update},
    prelude::ResMut,
};
use bevy_renet::{
    netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport},
    renet::{ConnectionConfig, DefaultChannel, RenetClient},
    RenetClientPlugin,
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
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
            .add_systems(Startup, send_message)
            .add_systems(Update, receive_message_system);
    }
}

fn send_message(mut client: ResMut<RenetClient>) {
    client.send_message(DefaultChannel::ReliableOrdered, "hello server");
}

fn receive_message_system(mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        println!("got message from server: {:?}", message);
    }
}
