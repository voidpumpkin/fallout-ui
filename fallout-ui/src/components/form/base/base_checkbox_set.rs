use std::collections::HashSet;
use std::ops::Deref;

use yew::prelude::*;

use crate::components::form::base::base_checkbox_input::BaseCheckboxInput;
use crate::components::form::fieldset::Fieldset;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub legend: String,
    pub available: Vec<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub checked: Vec<String>,
    #[prop_or_default]
    pub disabled: Vec<String>,
    #[prop_or_default]
    pub onchange: Callback<HashSet<String>>,
    #[prop_or_default]
    pub data_qa: String,
    #[prop_or_default]
    pub error: Option<String>,
}

#[function_component]
pub fn BaseCheckboxSet(props: &Props) -> Html {
    let Props {
        legend,
        required,
        checked,
        available,
        disabled,
        onchange,
        data_qa,
        error,
    } = props.clone();

    let checkboxes = available.deref().iter().map(|option| {
        let data_qa = data_qa.clone();
        let handle_on_change = {
            let checked = checked.clone();
            let onchange = onchange.clone();
            let option = option.clone();
            Callback::from(move |(value, _): (bool, Event)| {
                let option = option.clone();
                let mut changed_input: HashSet<String> = HashSet::from_iter(checked.clone());

                if value {
                    changed_input.insert(option);
                } else {
                    changed_input.remove(&option);
                }

                onchange.emit(changed_input);
            })
        };

        let checked = checked.contains(option);
        let disabled = disabled.contains(option);

        html! { <BaseCheckboxInput label={option.clone()} {checked} {disabled} onchange={handle_on_change} {data_qa}/> }
    });

    let legend = if required {
        format!("{legend}*")
    } else {
        legend
    };

    html! {
         <>
            <Fieldset {legend} {error} compact={true} data_qa={format!("{data_qa}-checkboxes")}>
                {for checkboxes.clone()}
            </Fieldset>
        </>
    }
}
