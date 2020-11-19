use syn::{parse_macro_input, DeriveInput};

mod derives;
mod util;

#[proc_macro_derive(Method, attributes(txkit))]
pub fn method(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse input tokens
    let input = parse_macro_input!(input as DeriveInput);
    derives::method::process_method(input).unwrap().into()
}

#[proc_macro_derive(ParamsFor, attributes(txkit))]
pub fn params_for(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse input tokens
    let input = parse_macro_input!(input as DeriveInput);
    derives::params_for::process_params_for(input)
        .unwrap()
        .into()
}
