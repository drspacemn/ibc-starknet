use starknet_ibc_core::client::{Height, Timestamp};
use starknet_ibc_core::host::{ChannelId, PortId, Sequence};

#[derive(Clone, Debug, Drop, Serde)]
pub struct Packet {
    pub seq_on_a: Sequence,
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub chan_id_on_b: ChannelId,
    pub data: Array<felt252>,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
}
