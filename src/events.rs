use crate::chat::message::HostMessage;
use crate::stats::Stats;
use wg_2024::packet::Packet;

#[derive(Debug, Clone)]
pub enum HostEvent {
    HostMessageSent(HostMessage),
    HostMessageReceived(HostMessage),
    StatsResponse(Stats),
    ControllerShortcut(Packet),
}
