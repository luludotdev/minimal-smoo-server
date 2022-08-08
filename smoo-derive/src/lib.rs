use proc_macro::{self, TokenStream};

mod packet_derive;

#[proc_macro_derive(Packet, attributes(packet))]
pub fn derive(input: TokenStream) -> TokenStream {
    crate::packet_derive::packet_derive(input)
}
