use std::ops::Deref;

use serde::Deserialize;
use serde::Serialize;
use yew::prelude::*;

use crate::components::form::base::base_input::BaseInput;
use crate::components::form::fieldset::Fieldset;
use crate::utils::form::FieldControlProps;
use crate::utils::toasts::notify_err;
use crate::utils::web_error::web_err_logic;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub legend: String,
    pub input_label: String,
    pub field_control_props: FieldControlProps<MultipleValues>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct MultipleWordsInputError {
    pub error: Option<String>,
    pub field_errors: Vec<Option<String>>,
}

impl MultipleWordsInputError {
    pub fn default_from_field_values(values: &MultipleValues) -> Self {
        MultipleWordsInputError {
            error: None,
            field_errors: values.iter().map(|_| None).collect(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
/// Wrapper over Vec<String> to enforce that the vector would always end with an empty string
pub struct MultipleValues(Vec<String>);

impl FromIterator<String> for MultipleValues {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<String>>().into()
    }
}

impl From<Vec<String>> for MultipleValues {
    fn from(mut values: Vec<String>) -> Self {
        if values.last().map(|last| !last.is_empty()).unwrap_or(true) {
            values.push("".to_string());
        }
        Self(values)
    }
}

impl Deref for MultipleValues {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MultipleValues {
    fn default() -> Self {
        Self(vec!["".to_string()])
    }
}

impl MultipleValues {
    pub fn push<T: ToString>(&mut self, v: T) {
        self.0.push(v.to_string());
        if !v.to_string().is_empty() {
            self.0.push("".to_string());
        }
    }

    pub fn into_inner(self) -> Vec<String> {
        self.0
    }

    pub fn into_submittable(mut self) -> Vec<String> {
        self.0.pop();
        self.0
    }
}

fn transform_values_on_value_change(
    values: MultipleValues,
    value: String,
    focused_id: Option<usize>,
    input_id: usize,
) -> Vec<String> {
    let mut values = values.into_inner();
    match values.get_mut(input_id) {
        Some(modifiable_value) => {
            *modifiable_value = value;
        }
        None => return values,
    }

    match focused_id {
        Some(focused_id)
            if focused_id == input_id
                && input_id != values.len().checked_sub(2).unwrap_or_default() =>
        {
            values
        }
        _ => values.into_iter().filter(|v| !v.is_empty()).collect(),
    }
}

#[function_component]
pub fn MultipleWordsInput(props: &Props) -> Html {
    let Props {
        legend,
        input_label,
        field_control_props:
            FieldControlProps {
                value: values,
                error,
                onchange,
                onblur,
            },
        disabled,
        required,
    } = props.clone();

    let focused_id_handle = use_state(Option::<usize>::default);
    let focused_id = *focused_id_handle;

    let MultipleWordsInputError {
        error,
        field_errors,
    } = match error {
        None => MultipleWordsInputError::default_from_field_values(&values),
        Some(errs) => match serde_json::from_str(errs.as_str()) {
            Ok(ok) => ok,
            Err(err) => {
                notify_err(web_err_logic(err));
                MultipleWordsInputError::default_from_field_values(&values)
            }
        },
    };

    let inputs_html: Html = values
        .clone()
        .into_inner()
        .into_iter()
        .zip(field_errors.into_iter())
        .enumerate()
        .map(|(i, (value, error))| {
            let onfocus = {
                let focused_id_handle = focused_id_handle.clone();

                Callback::from(move |_| focused_id_handle.set(Some(i)))
            };

            let onblur = {
                let values = values.clone();
                let onchange = onchange.clone();
                let focused_id_handle = focused_id_handle.clone();

                onblur.reform(move |_| {
                    let values = values
                        .clone()
                        .into_inner()
                        .into_iter()
                        .filter(|v| !v.is_empty())
                        .collect();
                    focused_id_handle.set(None);
                    onchange.emit(values);
                })
            };

            let handle_input = {
                let values = values.clone();
                let onchange = onchange.clone();

                Callback::from(move |(value, _): (String, _)| {
                    let value = value.trim_start().to_string();

                    onchange.emit(
                        transform_values_on_value_change(values.clone(), value, focused_id, i)
                            .into(),
                    );
                })
            };

            let handle_change = {
                let values = values.clone();
                let onchange = onchange.clone();

                Callback::from(move |(value, _): (String, _)| {
                    let value = value.trim().to_string();
                    onchange.emit(
                        transform_values_on_value_change(values.clone(), value, focused_id, i)
                            .into(),
                    );
                })
            };

            html! {
                <BaseInput
                    key={i}
                    id={format!{"{legend}-{input_label}-{i}-input"}}
                    label={input_label.clone()}
                    label_key={i.to_string()}
                    {onfocus}
                    {value}
                    {error}
                    onchange={handle_change}
                    {onblur}
                    oninput={handle_input}
                    {disabled}
                />
            }
        })
        .collect();

    let legend: String = if required {
        format!("{legend}*")
    } else {
        legend
    };

    html! {
        <Fieldset {legend} {error}>
            {inputs_html}
        </Fieldset>
    }
}
