use std::io::{Read, Write};

use packet::Packet;

mod encoding;
mod packet;

pub struct HandshakePacket;

impl Read for HandshakePacket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

impl Write for HandshakePacket {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl Packet for HandshakePacket {}
