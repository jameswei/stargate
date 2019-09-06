use network_libp2p::CustomMessage;
use parity_codec::{Encode, Decode};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub enum Message {
    ACK(u64),
    Payload(PayloadMsg),
}

#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct PayloadMsg {
    pub id: u64,
    pub data: Vec<u8>,
}

impl CustomMessage for Message
    where Self: Decode + Encode
{
    fn into_bytes(self) -> Vec<u8> {
        self.encode()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, ()> where Self: Sized {
        Decode::decode(&mut &bytes[..]).ok_or(())
    }
}

impl Message {
    pub fn new_ack(message_id: u64) -> Message {
        Message::ACK(message_id)
    }

    pub fn new_payload(data: Vec<u8>) -> (Message, u64) {
        let message_id = get_unix_ts();
        (Message::Payload(PayloadMsg { id: message_id, data }), message_id)
    }
}

fn get_unix_ts() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_millis() as u64
}