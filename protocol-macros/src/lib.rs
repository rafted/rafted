use minecraft_data_rs::{
    models::protocol::{PacketGrouping, PacketTypes},
    Api,
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

type WeirdTokenStream = proc_macro2::TokenStream;

#[proc_macro]
pub fn impl_structs(_input: TokenStream) -> TokenStream {
    let mut all_structs: Vec<WeirdTokenStream> = vec![];

    // Get an instance of the API to access the data of the latest minecraft version
    // TODO: Make the version configurable (via macro args)
    let api = Api::latest().expect("failed to retrieve latest version");
    let protocol = api
        .protocols
        .get_protocol()
        .expect("failed to get protocol section");

    // Get a list of all packet groupings. These represent the 4 states that we have, thus the name
    // of the variable.
    let states: Vec<(String, &PacketGrouping)> = vec![
        ("handshake".to_string(), &protocol.handshaking),
        ("login".to_string(), &protocol.login),
        ("status".to_string(), &protocol.status),
        ("play".to_string(), &protocol.play),
    ];

    // Go through each state, so we can create a module of structs for each one of them.
    for (state_name, state) in states {
        let state_name_ident = syn::Ident::new(&state_name, Span::call_site());
        let mut state_structs: Vec<WeirdTokenStream> = vec![];

        let directions: Vec<(String, &PacketTypes)> = vec![
            ("clientbound".to_string(), &state.to_client),
            ("serverbound".to_string(), &state.to_server),
        ];

        // Go through each direction in the state, so we can create a module for each one of them.
        for (direction_name, packets) in directions {
            let direction_name_ident = syn::Ident::new(&direction_name, Span::call_site());
            let mut direction_structs: Vec<WeirdTokenStream> = vec![];

            // Go through each packet and create a struct for it
            for packet in &packets.types {
                // Format the name to PascalCase so it is appropriate for a struct name
                let name = &packet.name.trim_start_matches("packet_");
                let fmt_name = voca_rs::case::pascal_case(name);
                let name_ident = syn::Ident::new(&fmt_name, Span::call_site());

                direction_structs.push(quote! {
                    pub struct #name_ident { }
                })
            }

            state_structs.push(quote! {
                pub mod #direction_name_ident {
                    #(#direction_structs)*
                }
            });
        }

        // Create a module for the state, to wrap all the packet structs that we just created
        all_structs.push(quote! {
            pub mod #state_name_ident {
                #(#state_structs)*
            }
        });
    }

    quote! {
        #(#all_structs)*
    }
    .into()
}

// #[proc_macro]
// pub fn impl_parse(_: TokenStream) -> TokenStream {
//     let stream = TokenStream::new();
//
//     // Get an instance of the API to access the data of the latest minecraft version
//     // TODO: Make the version configurable (via macro args)
//     let api = Api::latest().expect("failed to retrieve latest version");
//     let protocol = api.protocols.get_protocol().expect("failed to get protocol section");
//
//     // Get a list of all packet groupings. These represent the 4 states that we have, thus the name
//     // of the variable.
//     let states: Vec<&PacketGrouping> = vec![
//         &protocol.handshaking,
//         &protocol.login,
//         &protocol.status,
//         &protocol.play,
//     ];
//
//     // Go through each state, so we can create a case for each one of them
//     for state in states {
//
//         // We get the packets. Note that we only get the packets which are sent to the sever. This
//         // is because we're generating a parse function. On the write function, this should go
//         // through the to_client packets.
//         let packets = &state.to_server;
//
//         for packet in &packets.types {
//             let name = &packet.name.trim_start_matches("packet_");
//
//             // We get the ID of the packet. This is tricky because the schema that we're reading
//             // uses a "mappings" field, and it maps IDs to packet names, instead of the opposite.
//             // Also IDs are in hexadecimal, and strings.
//             let id_str: &String = (&packets.packet_mapper.mapper.mappings)
//                 .iter()
//                 .filter(|(_, c)| (*c).eq(name))
//                 .take(1)
//                 .next()
//                 .unwrap()
//                 .0;
//
//             let id = i32::from_str_radix(&id_str.trim_start_matches("0x"), 16).unwrap();
//
//             // read the fields
//             if let PacketDataType::Built{ value, .. } = &packet.data {
//                 dbg!(value);
//             }
//
//         }
//
//     }
//
//     stream
// }
