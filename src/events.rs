use crate::chat::message::HostMessage;
use crate::PacketHeader;
use std::time::Duration;
use wg_2024::packet::Packet;

#[derive(Debug, Clone)]
pub enum HostEvent {
    HostMessageSent(HostMessage, Duration),
    PacketSent(PacketHeader),
    ControllerShortcut(Packet),
}
