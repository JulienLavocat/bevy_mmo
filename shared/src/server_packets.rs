use bevy::{math::Vec3, prelude::Event};
use serde::{Deserialize, Serialize};

use crate::events_set::*;
use crate::packets_set;

#[derive(Serialize, Deserialize, Debug, Event)]
pub struct SpawnPlayer {
    pub location: Vec3,
}

packets_set!(
    ServerPacket,
    handle_server_packet,
    ServerPacketsWriter { SpawnPlayer }
);
