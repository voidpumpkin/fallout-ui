use std::collections::HashSet;
use std::ops::Deref;

use clone_on_capture::clone_on_capture;
use yew::prelude::*;

use crate::components::buttons::outlined_secondary_button::OutlinedSecondaryButton;
use crate::components::form::base::base_checkbox_set::BaseCheckboxSet;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[clone_on_capture]
#[function_component]
pub fn BaseCheckboxSetStory() -> Html {
    let values_handle: UseStateHandle<HashSet<String>> = use_state_eq(Default::default);
    let values: Vec<String> = values_handle.deref().clone().into_iter().collect();

    let handle_change = move |checked| values_handle.set(checked);

    let available = vec![
        "Apples".to_string(),
        "Bananas".to_string(),
        "Blueberries".to_string(),
        "Disabled".to_string(),
    ];
    let check_all = move |_| values_handle.set(available.clone().into_iter().collect());

    let disabled = vec!["Disabled".to_string()];

    html! {
        <Stories>
            <Story name={"Plain"} background={StoryBackground::White}>
                <BaseCheckboxSet legend={"Fruits"} available={available.clone()} disabled={disabled.clone()} checked={values.clone()} onchange={handle_change.clone()}/>
            </Story>
            <Story name={"Required"} background={StoryBackground::White}>
                <BaseCheckboxSet legend={"Required Fruits"} available={available.clone()} disabled={disabled.clone()} checked={values.clone()} onchange={handle_change.clone()} required={true}/>
            </Story>
            <Story name={"Disabled All"} background={StoryBackground::White}>
                <BaseCheckboxSet legend={"Disabled All Fruits"} available={available.clone()} checked={values.clone()} onchange={handle_change.clone()} disabled={available.clone()}/>
            </Story>
            <Story name={"With Error"} background={StoryBackground::White}>
                <BaseCheckboxSet legend={"Invalid Fruits"} available={available} disabled={disabled} checked={values} onchange={handle_change} error={"This is an Error"}/>
            </Story>
            <OutlinedSecondaryButton onclick={check_all}>{"Check All"}</OutlinedSecondaryButton>
        </Stories>
    }
}
