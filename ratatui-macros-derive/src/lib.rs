use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn vertical(args: TokenStream, input: TokenStream) -> TokenStream {
    todo!()
    // macro(args.into(), input.into()).into()
}
