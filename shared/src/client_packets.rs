use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::events_set::*;
use crate::packets_set;

#[derive(Serialize, Deserialize, Debug, Event)]
pub struct Noop {
    pub test: bool,
}

packets_set!(
    ClientPacket,
    handle_client_packet,
    ClientPacketEventsWriter { Noop }
);
