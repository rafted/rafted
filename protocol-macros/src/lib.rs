use minecraft_data_rs::Api;
use proc_macro::TokenStream;

#[proc_macro]
pub fn impl_parse(item: TokenStream) -> TokenStream {
    let stream = TokenStream::new();

    let api = Api::latest().expect("failed to retrieve latest version");
    let protocol = api.protocols.get_protocol().expect("failed to get protocol section");

    for ele in protocol.handshaking.to_server.types {
        dbg!(ele);
    }

    stream
}
