extern crate proc_macro;

mod form_fields;
mod utils;

use syn::parse_macro_input;
use syn::DeriveInput;

use crate::proc_macro::TokenStream;

#[proc_macro_derive(FormFields, attributes(map_err))]
pub fn form_fields(item: TokenStream) -> TokenStream {
    form_fields::form_fields_impl(parse_macro_input!(item as DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
