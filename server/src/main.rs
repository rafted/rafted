use std::error::Error;

use protocol_macros::impl_structs;

impl_structs!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
