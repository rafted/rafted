use proc_macro::TokenStream;

#[proc_macro_derive(Packet)]
fn derive_packet(item: TokenStream) -> TokenStream {
    let stream = TokenStream::new();

    stream
}
