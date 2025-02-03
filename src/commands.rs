use crossbeam_channel::Sender;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

#[derive(Debug, Clone)]
pub enum HostCommand {
    SendRandomMessage(NodeId),
    DiscoverNetwork,
    StatsRequest,
    AddSender(NodeId, Sender<Packet>),
    RemoveSender(NodeId),
    Stop,
}
