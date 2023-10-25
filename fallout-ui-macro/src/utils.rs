use quote::format_ident;
use syn::Ident;

pub fn create_idents<T, F>(arr: &[T], mut f: F) -> Vec<Ident>
where
    F: FnMut(&T) -> String,
{
    arr.iter()
        .map(|field_name| format_ident!("{}", f(field_name)))
        .collect()
}
