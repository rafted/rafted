use std::collections::HashMap;

use minecraft_data_rs::{Api, models::protocol::PacketGrouping};
use proc_macro::TokenStream;

#[proc_macro]
pub fn impl_parse(_: TokenStream) -> TokenStream {
    let stream = TokenStream::new();

    // Get an instance of the API to access the data of the latest minecraft version
    // TODO: Make the version configurable (via macro args)
    let api = Api::latest().expect("failed to retrieve latest version");
    let protocol = api.protocols.get_protocol().expect("failed to get protocol section");

    // Get a list of all packet groupings. These represent the 4 states that we have, thus the name
    // of the variable.
    let states: Vec<&PacketGrouping> = vec![
        &protocol.handshaking,
        &protocol.login,
        &protocol.status,
        &protocol.play,
    ];

    // Go through each state, so we can create a case for each one of them 
    for state in states {

        // We get the packets. Note that we only get the packets which are sent to the sever. This
        // is because we're generating a parse function. On the write function, this should go
        // through the to_client packets.
        let packets = &state.to_server;

        for packet in &packets.types {
            let name = &packet.name.trim_start_matches("packet_");

            // We get the ID of the packet. This is tricky because the schema that we're reading
            // uses a "mappings" field, and it maps IDs to packet names, instead of the opposite.
            // Also IDs are in hexadecimal, and strings.
            let id_str: &String = (&packets.packet_mapper.mapper.mappings)
                .iter()
                .filter(|(_, c)| (*c).eq(name))
                .take(1)
                .next()
                .unwrap()
                .0;

            let id = i32::from_str_radix(&id_str.trim_start_matches("0x"), 16).unwrap();

            dbg!(name, id_str, id);

        }

    }

    stream
}
