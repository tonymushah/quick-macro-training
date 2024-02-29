use proc_macro::{self, TokenStream};
use syn::{parse as syn_parse, Item, parse_macro_input, Attribute};

#[proc_macro_attribute]
pub fn tony(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    match item {
        Item::Fn(fun) => {
            fun.sig.fn_token.
        }
        _ => {
            panic!("Excepted function")
        }
    }
}