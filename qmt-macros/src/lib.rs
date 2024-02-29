use proc_macro::{self, TokenStream};

#[proc_macro]
pub fn my_macro(token: TokenStream) -> TokenStream {
    token
}