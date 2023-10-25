use crate::utils::create_idents;
use convert_case::Case;
use convert_case::Casing;
use quote::format_ident;
use quote::quote;
use syn::Ident;
use syn::Result;

pub fn hooks_quote(
    FormSchema: &Ident,
    Field: &Ident,
    FieldsControlProps: &Ident,
    Action: &Ident,
    FieldSetValuePayload: &Ident,
    static_field_ident: &[Ident],
    StaticField: &[Ident],
    dynamic_field_ident: &[Ident],
    DynamicField: &[Ident],
) -> Result<proc_macro2::TokenStream> {
    let use_form_schema: Ident =
        format_ident!("use_{}", FormSchema.to_string().to_case(Case::Snake));

    let handle_static_field_change: Vec<Ident> =
        create_idents(static_field_ident, |static_field_ident| {
            format!("handle_{static_field_ident}_change")
        });

    let handle_static_field_blur: Vec<Ident> =
        create_idents(static_field_ident, |static_field_ident| {
            format!("handle_{static_field_ident}_blur")
        });

    let handle_dynamic_field_change: Vec<Ident> =
        create_idents(dynamic_field_ident, |dynamic_field_ident| {
            format!("handle_{dynamic_field_ident}_change")
        });

    let handle_dynamic_field_blur: Vec<Ident> =
        create_idents(dynamic_field_ident, |dynamic_field_ident| {
            format!("handle_{dynamic_field_ident}_blur")
        });

    Ok(quote! {
        #[hook]
        pub fn #use_form_schema<F>(default_value: F) -> (UseReducerHandle<crate::utils::form_schema_handle::FormSchemaHandle<#FormSchema, #Field>>, #FieldsControlProps)
        where
            F: FnOnce() -> crate::utils::form_schema_handle::FormSchemaHandle<#FormSchema, #Field>,
        {
            use std::collections::HashMap;
            use fallout_ui::utils::form::FieldControlProps;
            use fallout_ui::utils::form::DynamicFieldControlProps;

            let form_handle = use_reducer(default_value);
            let dispatcher = form_handle.dispatcher();

            #(
                let #handle_static_field_change = use_callback(
                    dispatcher.clone(),
                    move |value, dispatcher| dispatcher.dispatch(#Action::SetValue(#FieldSetValuePayload::#StaticField(value))),
                );

                let #handle_static_field_blur = use_callback(
                    dispatcher.clone(),
                    move |(), dispatcher| dispatcher.dispatch(#Action::Touch(#Field::#StaticField)),
                );

                // Do not show the error if a field is not touched yet
                let error = use_memo(
                    (
                        form_handle.errors.get(&#Field::#StaticField).cloned(),
                        form_handle.touched_fields.get(&#Field::#StaticField).is_some()
                    ),
                    move |(error, is_touched)| is_touched.then(|| error.clone()).flatten(),
                ).as_ref().clone();

                let #static_field_ident = use_memo(
                    (
                        form_handle.values.#static_field_ident.clone(),
                        error,
                        #handle_static_field_change,
                        #handle_static_field_blur,
                    ),
                    move |(value, error, #handle_static_field_change, #handle_static_field_blur,)| {
                        FieldControlProps {
                            value: value.clone(),
                            error: error.clone(),
                            onchange: #handle_static_field_change.clone(),
                            onblur: #handle_static_field_blur.clone(),
                        }
                    },
                ).as_ref().clone();
            )*

            #(
                let #handle_dynamic_field_change = use_callback(
                    dispatcher.clone(),
                    move |(key, value), dispatcher| dispatcher.dispatch(#Action::SetValue(#FieldSetValuePayload::#DynamicField(key, value))),
                );

                let #handle_dynamic_field_blur = use_callback(
                    dispatcher.clone(),
                    move |key, dispatcher| dispatcher.dispatch(#Action::Touch(#Field::#DynamicField(Some(key)))),
                );

                // Do not show the error if a field is not touched yet
                let error = use_memo(
                    (
                        form_handle
                            .errors
                            .iter()
                            .filter_map(|(key, err)| match &key {
                                &#Field::#DynamicField(key) => Some((key.clone()?, err.clone())),
                                _ => None,
                            })
                            .collect::<HashMap<_, _>>(),
                        form_handle
                            .touched_fields
                            .iter()
                            .filter_map(|touched_field| match &touched_field {
                                &#Field::#DynamicField(key) => Some(key.clone()?),
                                _ => None,
                            })
                            .collect::<Vec<_>>(),
                        form_handle
                            .touched_fields
                            .iter()
                            .any(|err| matches!(err, &#Field::#DynamicField(None))),
                    ),
                    move |(errors, touched_fields, all_touched)| {
                        if *all_touched {
                            return errors.clone();
                        }

                        touched_fields
                            .iter()
                            .filter_map(|touched_field| Some((touched_field.clone(), errors.get(touched_field).cloned()?)))
                            .collect::<HashMap<_, _>>()
                    },
                ).as_ref().clone();

                let #dynamic_field_ident = use_memo(
                    (
                        form_handle.values.#dynamic_field_ident.clone(),
                        error,
                        #handle_dynamic_field_change,
                        #handle_dynamic_field_blur,
                    ),
                    move |(value, error, #handle_dynamic_field_change, #handle_dynamic_field_blur,)| {
                        DynamicFieldControlProps {
                            value: value.clone(),
                            error: error.clone(),
                            onchange: #handle_dynamic_field_change.clone(),
                            onblur: #handle_dynamic_field_blur.clone(),
                        }
                    },
                ).as_ref().clone();
            )*

            let FieldsControlProps = #FieldsControlProps {
                #(
                    #static_field_ident,
                )*
                #(
                    #dynamic_field_ident,
                )*
            };

            (form_handle, FieldsControlProps)
        }
    })
}
