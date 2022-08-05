use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(PacketBytes)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let writers = fields.clone().into_iter().map(|f| {
        let field_name = f.ident;

        quote! {
            written += self.#field_name.write_bytes(buf);
        }
    });

    let readers = fields.into_iter().map(|f| {
        let field_name = f.ident;
        let field_ty = f.ty;

        quote! {
            #field_name: <#field_ty as crate::packet::PacketBytes>::from_bytes(buf)?,
        }
    });

    let output = quote! {
        #[automatically_derived]
        impl crate::packet::PacketBytes for #ident {
            fn write_bytes(&self, buf: &mut bytes::BytesMut) -> usize {
                let mut written = 0;
                #(#writers)*
                written
            }

            fn from_bytes<T: bytes::Buf>(buf: &mut T) -> color_eyre::Result<Self> {
                Ok(Self {
                    #(#readers)*
                })
            }
        }
    };

    output.into()
}
