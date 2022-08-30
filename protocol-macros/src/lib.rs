use minecraft_data_rs::{
    models::protocol::{types::TypeName, NativeType, PacketDataType, PacketGrouping, PacketTypes},
    Api,
};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

type WeirdTokenStream = proc_macro2::TokenStream;

// helper macros
macro_rules! unpack_built {
    ($x:ident) => {
        if let PacketDataType::Built { name: _, value } = $x {
            value
        } else {
            panic!("not built")
        }
    };
}

fn convert_type(t: &PacketDataType) -> Ident {
    let type_ = match t {
        PacketDataType::Native(v) => match v {
            NativeType::VarInt => "i32",
            NativeType::PString { count_type: _ } => "String",
            NativeType::Buffer { count_type: _ } => todo!("Buffer"),
            NativeType::Bool => "bool",
            NativeType::U8 => "u8",
            NativeType::U16 => "u16",
            NativeType::U32 => "u32",
            NativeType::U64 => "u64",
            NativeType::I8 => "i8",
            NativeType::I16 => "i16",
            NativeType::I32 => "i32",
            NativeType::I64 => "i64",
            NativeType::F32 => "f32",
            NativeType::F64 => "f64",
            NativeType::Uuid => "uuid::Uuid",
            NativeType::Option(_) => todo!("Option"),
            NativeType::EntityMetadataLoop {
                end_val: _,
                metadata_type: _,
            } => todo!("EntityMetadataLoop"),
            NativeType::TopBitSetTerminatedArray(_) => todo!("BitSet"),
            NativeType::BitField(_) => todo!("BitField"),
            NativeType::Container(_) => todo!("Container"),
            NativeType::Switch {
                compare_to: _,
                fields: _,
                default: _,
            } => todo!("Switch"),
            NativeType::Void => todo!("Void"),
            NativeType::Array {
                count_type: _,
                array_type: _,
            } => todo!("Array"),
            NativeType::RestBuffer => todo!("RestBuffer"),
            NativeType::NBT => todo!("NBT"),
            NativeType::OptionalNBT => todo!("OptionalNBT"),
            _ => todo!(),
        },
        PacketDataType::UnknownNativeType(_) => {
            todo!("UnknownNativeType")
        }
        PacketDataType::Built { name: _, value: _ } => {
            todo!("Built")
        }
        PacketDataType::Other { name: _, value: _ } => {
            todo!("Other")
        }
    };

    syn::Ident::new(type_, Span::call_site())
}

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

                // Create the list of fields for the struct
                // God this is messy...
                let mut fields: Vec<WeirdTokenStream> = vec![];

                let data = &packet.data;
                let data = unpack_built!(data);

                if let NativeType::Container(v) = &data {
                    for field in v {
                        // get the name of the field
                        let name = if let TypeName::Named(name) = &field.0 {
                            Some(name)
                        } else {
                            None
                        }
                        .unwrap();

                        let mut fmt_name = voca_rs::case::snake_case(name);

                        // blacklist reserved types
                        if fmt_name == "type" {
                            fmt_name = "type_".to_string();
                        }

                        let name_ident = syn::Ident::new(&fmt_name, Span::call_site());

                        // get the type of the field
                        let type_ident = convert_type(&*field.1);

                        fields.push(quote! {
                            pub #name_ident: #type_ident
                        });
                    }
                }

                direction_structs.push(quote! {
                    pub struct #name_ident {
                        #(#fields),*
                    }
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
