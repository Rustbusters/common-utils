use wg_2024::network::SourceRoutingHeader;

#[derive(Clone, Debug, Default)]
pub struct PacketHeader {
    pub routing_header: SourceRoutingHeader,
    pub session_id: u64,
    pub pack_type: PacketTypeHeader,
}

#[derive(Clone, Debug, Default)]
pub enum PacketTypeHeader {
    #[default]
    MsgFragment,
    Ack,
    FloodRequest,
    FloodResponse,
}
