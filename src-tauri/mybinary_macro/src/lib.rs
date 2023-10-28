use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn command(attr: TokenStream, body: TokenStream) -> TokenStream {
    implement::command(attr.into(), body.into()).into()
}
