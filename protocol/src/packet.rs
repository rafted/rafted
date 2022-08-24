use std::io::{Read, Write};

pub trait Packet: Read + Write + Send { }
