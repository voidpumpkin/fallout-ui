#![allow(clippy::type_complexity)]
#![allow(clippy::cmp_owned)]
#![allow(clippy::too_many_arguments)]
// Most variables hold struct Idents or rust types,
// so it is more readable when we keep them Pascal cased
#![allow(non_snake_case)]

pub mod hooks;
pub mod split_away_dynamic_fields;

use crate::utils::create_idents;
use convert_case::Case;
use convert_case::Casing;
use proc_macro::Span;
use quote::format_ident;
use quote::quote;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Error;
use syn::Fields;
use syn::Ident;
use syn::LitStr;
use syn::Result;
use syn::Type;
use syn::TypePath;

use self::hooks::hooks_quote;
use self::split_away_dynamic_fields::split_away_dynamic_fields;

extern crate proc_macro;

const IS_NOT_NAMED_FIELD: &str = "Only named fields allowed";
const IS_NOT_STRUCT: &str = "Only structs are allowed to derive this macro";
const HASHMAP_NOT_SUPPORTED: &str = "HashMaps are not supported, use BTreeMap Instead";

pub fn form_fields_impl(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let DeriveInput {
        ident: FormSchema,
        data,
        ..
    } = input;

    let Data::Struct(DataStruct { fields, .. }) = data else {
        return Err(Error::new(FormSchema.span(), IS_NOT_STRUCT));
    };

    let Fields::Named(named_fields) = fields else {
        panic!("{IS_NOT_NAMED_FIELD}");
    };

    for field in named_fields.named.iter() {
        if let Type::Path(TypePath { path, .. }) = &field.ty {
            if let Some(segment) = path.segments.first() {
                if segment.ident.to_string() == "HashMap".to_string() {
                    panic!("{HASHMAP_NOT_SUPPORTED}");
                }
            }
        }
    }

    let (static_named_fields, (dynamic_named_fields, DynamicFieldKeyType, DynamicFieldValueType)) =
        split_away_dynamic_fields(named_fields.clone());

    // Static Fields Idents
    let static_field: Vec<Ident> = static_named_fields
        .iter()
        .map(|field| field.ident.clone().expect(IS_NOT_NAMED_FIELD))
        .collect();

    let StaticFieldType: Vec<Type> = static_named_fields
        .iter()
        .map(|field| field.ty.clone())
        .collect();

    let StaticField: Vec<Ident> = create_idents(&static_field, |field| {
        field.to_string().to_case(Case::Pascal)
    });

    let static_literal_field_name: Vec<LitStr> = static_field
        .iter()
        .map(|field| LitStr::new(field.to_string().as_str(), Span::call_site().into()))
        .collect();

    // Dynamic Fields Idents
    let dynamic_field: Vec<Ident> = dynamic_named_fields
        .iter()
        .map(|field| field.ident.clone().expect(IS_NOT_NAMED_FIELD))
        .collect();

    let DynamicField: Vec<Ident> = create_idents(&dynamic_field, |field| {
        field.to_string().to_case(Case::Pascal)
    });

    let DynamicFieldType: Vec<Type> = dynamic_named_fields
        .iter()
        .map(|field| field.ty.clone())
        .collect();

    let dynamic_literal_field_name: Vec<LitStr> = dynamic_field
        .iter()
        .map(|field| LitStr::new(field.to_string().as_str(), Span::call_site().into()))
        .collect();

    // Other Idents
    let Field = format_ident!("{FormSchema}Field");
    let FieldSetValuePayload = format_ident!("{FormSchema}FieldSetValuePayload");
    let Action = format_ident!("{FormSchema}Action");
    let FieldsControlProps: Ident = format_ident!("{FormSchema}FieldsControlProps");

    // Quotes
    let hooks_quote = hooks_quote(
        &FormSchema,
        &Field,
        &FieldsControlProps,
        &Action,
        &FieldSetValuePayload,
        &static_field,
        &StaticField,
        &dynamic_field,
        &DynamicField,
    )?;

    Ok(quote! {

            #[derive(Debug, Clone, Eq, PartialEq, Hash, strum::EnumString, strum::Display, strum::EnumIter)]
            pub enum #Field {
                #(
                    #[strum(serialize = #static_literal_field_name)]  // Used only for reading [validator] output
                    #StaticField,
                )*
                #(
                    #[strum(serialize = #dynamic_literal_field_name)]  // Used only for reading [validator] output
                    #DynamicField(Option<#DynamicFieldKeyType>), // None means All
                )*
            }

            #[derive(Debug, Clone, PartialEq)]
            pub enum #FieldSetValuePayload {
                #(
                    #StaticField(#StaticFieldType),
                )*
                #(
                    #DynamicField(#DynamicFieldKeyType, #DynamicFieldValueType),
                )*
            }

            impl From<&#FieldSetValuePayload> for #Field {
                fn from(value: &#FieldSetValuePayload) -> Self {
                    match value {
                        #(
                            #FieldSetValuePayload::#StaticField(_) => #Field::#StaticField,
                        )*
                        #(
                            #FieldSetValuePayload::#DynamicField(key, _) => #Field::#DynamicField(Some(key.clone())),
                        )*
                    }
                }
            }

            #[derive(Debug, Clone, PartialEq)]
            pub enum #Action {
                SetValue(#FieldSetValuePayload),
                OverrideValue(#FieldSetValuePayload),
                SetError(#Field, Option<String>),
                SetErrors(std::collections::HashMap<#Field, String>),
                Touch(#Field),
                TouchAll,
                Reset(crate::utils::form_schema_handle::FormSchemaHandle<#FormSchema, #Field>),
            }

            impl yew::prelude::Reducible for crate::utils::form_schema_handle::FormSchemaHandle<#FormSchema, #Field> {
                type Action = #Action;

                fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
                    let mut state = (*self).clone();

                    match action {
                        #Action::SetValue(payload) => {
                            let field: #Field = (&payload).into();
                            match payload {
                                #(
                                    #FieldSetValuePayload::#StaticField(value) => {
                                        let current_value = &state.values.#static_field;

                                        if current_value != &value {
                                            state.set_error(field.clone(), None);
                                            state.touched_fields.insert(field);
                                            state.values.#static_field = value;
                                            state.fallout_validate();
                                        }

                                    }
                                )*
                                #(
                                    #FieldSetValuePayload::#DynamicField(key, value) => {
                                        let current_value = state.values.#dynamic_field.get(&key);

                                        match state.values.#dynamic_field.get(&key) {
                                            Some(current_value) if current_value == &value => {},
                                            _ => {
                                                state.set_error(field.clone(), None);
                                                state.touched_fields.insert(field);
                                                if let Some(state_value) = state.values.#dynamic_field.get_mut(&key) {
                                                    *state_value = value;
                                                } else {
                                                    state.values.#dynamic_field.insert(key, value);
                                                }
                                                state.fallout_validate();

                                            }
                                        }
                                    }
                                )*
                            };

                            state.into()
                        }
                        #(
                            #Action::OverrideValue(#FieldSetValuePayload::#StaticField(value)) => {
                                state.values.#static_field = value;
                                state.into()
                            }
                        )*
                        #(
                            #Action::OverrideValue(#FieldSetValuePayload::#DynamicField(key, value)) => {
                                if let Some(state_value) = state.values.#dynamic_field.get_mut(&key) {
                                    *state_value = value;
                                } else {
                                    state.values.#dynamic_field.insert(key, value);
                                }
                                state.into()
                            }
                        )*
                        #Action::SetError(field, err) => {
                            state.set_error(field, err);
                            state.into()
                        }
                        #Action::Touch(field) => {
                            state.touched_fields.insert(field);
                            state.fallout_validate();
                            state.into()
                        }
                        #Action::TouchAll => {
                            #(
                                state.touched_fields.insert(#Field::#StaticField);
                            )*
                            #(
                                state.values.#dynamic_field
                                    .iter()
                                    .for_each(|(k,v)| {
                                        state.touched_fields.insert(#Field::#DynamicField(Some(k.clone())));
                                    });
                            )*
                            state.fallout_validate();
                            state.into()
                        }
                        #Action::SetErrors(errs) => {
                            for err in errs {
                                state.set_error(err.0, Some(err.1));
                            }
                            state.into()
                        }
                        #Action::Reset(new_state) => {
                            new_state.into()
                        }
                    }
                }
            }

            #[derive(Clone, PartialEq)]
            pub struct #FieldsControlProps {
                #(
                    pub #static_field: fallout_ui::utils::form::FieldControlProps<#StaticFieldType>,
                )*
                #(
                    pub #dynamic_field: fallout_ui::utils::form::DynamicFieldControlProps<#DynamicFieldType, #DynamicFieldKeyType, #DynamicFieldValueType>,
                )*
            }

            #hooks_quote
    })
}
