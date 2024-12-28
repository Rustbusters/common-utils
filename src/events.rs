use crate::stats::Stats;
use crate::chat::message::HostMessage;
use wg_2024::packet::Packet;

#[derive(Debug)]
pub enum HostEvent {
    HostMessageSent(HostMessage),
    HostMessageReceived(HostMessage),
    StatsResponse(Stats),
    ControllerShortcut(Packet),
}