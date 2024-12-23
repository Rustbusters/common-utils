use crate::stats::Stats;
use crate::HostMessage;
use wg_2024::packet::Packet;

#[derive(Debug, Clone)]
pub enum HostEvent<T: HostMessage> {
    MessageSent(T),
    MessageReceived(T),
    StatsResponse(Stats),
    ControllerShortcut(Packet),
}
