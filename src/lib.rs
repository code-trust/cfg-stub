use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ImplItem,
    ItemImpl,
    parse_macro_input,
    parse_quote,
};

#[proc_macro_attribute]
pub fn methods(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut imp = parse_macro_input!(item as ItemImpl);

    for item in &mut imp.items {
        if let ImplItem::Fn(m) = item {
            m.block = parse_quote!({ unimplemented!() });
        }
    }

    TokenStream::from(quote!(#imp))
}
