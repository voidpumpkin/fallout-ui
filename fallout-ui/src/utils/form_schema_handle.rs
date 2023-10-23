#[macro_export]
macro_rules! form_schema_handle_shared_code {
    () => {
        use std::collections::HashMap;
        use std::collections::HashSet;
        use std::hash::Hash;
        use std::str::FromStr;

        use strum::IntoEnumIterator;
        use validator::Validate;

        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Debug, Default, PartialEq, Clone)]
        pub struct FormSchemaHandle<S, F>
        where
            S: PartialEq,
            F: Eq + Hash,
        {
            pub values: S,
            pub errors: HashMap<F, String>,
            pub touched_fields: HashSet<F>,
        }

        impl<S, F> FormSchemaHandle<S, F>
        where
            S: Validate + Clone + PartialEq,
            F: Eq + Hash + FromStr + IntoEnumIterator,
        {
            pub fn new(initial_values: S) -> Self {
                let mut form_schema_handle = Self {
                    values: initial_values,
                    errors: HashMap::<F, String>::default(),
                    touched_fields: HashSet::<F>::default(),
                };
                form_schema_handle.fallout_validate();
                form_schema_handle
            }

            pub fn new_with_all_touched(initial_values: S) -> Self {
                let mut form_schema_handle = Self {
                    values: initial_values,
                    errors: HashMap::<F, String>::default(),
                    touched_fields: F::iter().collect(),
                };
                form_schema_handle.fallout_validate();
                form_schema_handle
            }

            pub fn set_error(&mut self, field: F, err: Option<String>) {
                match err {
                    Some(err) => self.errors.insert(field, err),
                    None => self.errors.remove(&field),
                };
            }

            pub fn has_errors(&self) -> bool {
                !self.errors.is_empty()
            }

            /// # !!! Careful
            /// This function is intended for use in constructor or in `FormFields` macro Reducer
            pub fn fallout_validate(&mut self) {
                let errors = match self.values.validate() {
                    Err(err) => err,
                    Ok(_) => return,
                };

                for (field_name, field_errs) in errors.field_errors().into_iter() {
                    let field_error = match field_errs.get(0) {
                        Some(some) => some,
                        None => continue,
                    };
                    let field = match F::from_str(field_name) {
                        Ok(ok) => ok,
                        Err(_) => continue,
                    };
                    let err = match &field_error.message {
                        Some(some) => some.to_string(),
                        None => "Invalid field".to_string(),
                    };
                    self.set_error(field, Some(err));
                }
            }
        }
    };
}

mod inner {
    form_schema_handle_shared_code!();
}
pub(crate) use inner::*;
