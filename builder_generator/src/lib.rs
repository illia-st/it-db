use proc_macro::TokenStream;

#[proc_macro_derive(ClosureBuilder)]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
