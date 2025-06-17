//! Infrared sender functionality

use crate::protocol::Protocol;

mod buffer;
mod hal;
mod senders;

pub use buffer::*;
pub use hal::*;
pub use senders::*;

pub trait ProtocolEncoder<const FREQ: u32>: Protocol {
    type EncoderData;
    const DATA: Self::EncoderData;

    fn encode(cmd: &Self::Cmd, buf: &mut [u32]) -> usize;
}
