use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Marks the main function to be executed within a terminal context.
///
/// This macro automatically initializes the terminal and passes it as an argument
/// to the main function, simplifying the setup without requiring the user to
/// handle terminal initialization and restoration directly.
///
/// # Example
///
/// ```rust
/// #[ratatui_macros::main]
/// fn main(mut terminal: Terminal<CrosstermBackend<Stdout>>) {
///     // draw on terminal
/// }
/// ```
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let block = &input.block;
    let output = quote! {
        fn main() {
            let mut terminal = ratatui::init();
            (|mut terminal: ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>| {
                #block
            })(terminal);
            ratatui::restore();
        }
    };
    TokenStream::from(output)
}
