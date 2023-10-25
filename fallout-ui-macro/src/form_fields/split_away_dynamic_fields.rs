use syn::Field;
use syn::FieldsNamed;
use syn::GenericArgument;
use syn::PathArguments;
use syn::Type;
use syn::TypePath;

pub fn split_away_dynamic_fields(
    named_fields: FieldsNamed,
) -> (Vec<Field>, (Vec<Field>, Vec<Type>, Vec<Type>)) {
    let dynamic_fields: (Vec<Field>, Vec<(Type, Type)>) = named_fields
        .named
        .clone()
        .into_iter()
        .filter_map(|field| {
            let Type::Path(TypePath { path, .. }) = field.ty.clone() else {
                return None;
            };

            let Some(segment) = path.segments.first() else {
                return None;
            };

            if segment.ident.to_string() != "BTreeMap".to_string() {
                return None;
            };

            let PathArguments::AngleBracketed(angle_bracketed) = &segment.arguments else {
                panic!("BTreeMap was not AngleBracketed");
            };

            let mut key_type = None;
            let mut value_type = None;

            let mut args_iter = angle_bracketed.args.iter();
            if let GenericArgument::Type(tp) = args_iter.next().expect("Expected BTreeMap generics")
            {
                key_type = Some(tp.clone());
            }
            if let GenericArgument::Type(tp) = args_iter.next().expect("Expected BTreeMap generics")
            {
                value_type = Some(tp.clone());
            }

            Some((
                field,
                (
                    key_type.expect("Failed to get generic types from BTreeMap"),
                    value_type.expect("Failed to get generic types from BTreeMap"),
                ),
            ))
        })
        .unzip();

    let unziped_key_values: (Vec<Type>, Vec<Type>) = dynamic_fields.1.into_iter().unzip();
    let dynamic_fields: (Vec<Field>, Vec<Type>, Vec<Type>) =
        (dynamic_fields.0, unziped_key_values.0, unziped_key_values.1);

    let static_fields: Vec<Field> = named_fields
        .named
        .into_iter()
        .filter(|field| !dynamic_fields.0.iter().any(|dyn_field| dyn_field == field))
        .collect();

    (static_fields, dynamic_fields)
}
