use wg_2024::packet::Packet;
use crate::chat::message::HostMessage;
use crate::PacketHeader;

#[derive(Debug, Clone)]
pub enum HostEvent {
    HostMessageSent(HostMessage),
    PacketSent(PacketHeader),
    ControllerShortcut(Packet),
}
