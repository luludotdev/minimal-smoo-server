use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, LitStr};

#[proc_macro_derive(Packet, attributes(packet))]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);

    let value = attrs
        .iter()
        .filter(|attr| attr.path.is_ident("packet"))
        .map(|attr| {
            let meta: LitStr = attr.parse_args().unwrap();
            meta
        })
        .next();

    let name = match value {
        Some(value) => value.parse::<syn::Type>().unwrap(),
        None => panic!("missing #[packet()] attribute"),
    };

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

        #[automatically_derived]
        impl From<#ident> for crate::packet::PacketData {
            #[inline(always)]
            fn from(packet: #ident) -> Self {
                Self::#name(packet)
            }
        }

        #[automatically_derived]
        impl crate::packet::IntoPacket for #ident {}
    };

    output.into()
}
