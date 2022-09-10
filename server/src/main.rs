use std::error::Error;

use net::connection::Connection;
use protocol_macros::impl_structs;
use tokio::net::TcpListener;

pub mod net;

impl_structs!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:25565").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        let connection = Connection::new(socket);

        connection.handle().await;
    }
}
