use tokio::net::TcpStream;

use super::state::State;

pub struct Connection {
    pub state: State,

    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            state: State::Handshaking,
            stream
        }
    }

    pub async fn handle(&self) {
        loop {
            // read data
        } 
    }
}
