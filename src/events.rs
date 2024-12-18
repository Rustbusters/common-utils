use wg_2024::packet::Packet;
use crate::messages::Message;
use crate::stats::Stats;

#[derive(Debug, Clone)]
pub enum HostEvent {
    MessageSent(Message),
    MessageReceived(Message),
    StatsResponse(Stats),
    ControllerShortcut(Packet),
}
