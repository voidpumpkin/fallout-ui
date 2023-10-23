use std::ops::Deref;

use clone_on_capture::clone_on_capture;
use yew::prelude::*;

use crate::components::buttons::outlined_secondary_button::OutlinedSecondaryButton;
use crate::components::form::base::base_checkbox_input::BaseCheckboxInput;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[clone_on_capture]
#[function_component]
pub fn BaseCheckboxStory() -> Html {
    let toggle_handle = use_state_eq(|| false);
    let toggle_checked = *toggle_handle.deref();

    let toggle_button = move |_| toggle_handle.set(!toggle_checked);
    let toggle_change = move |(checked, _): (bool, _)| toggle_handle.set(checked);

    html! {
        <Stories>
            <Story name={"Plain"} background={StoryBackground::White}>
                <BaseCheckboxInput label={"Apples"} checked={toggle_checked} onchange={toggle_change.clone()}/>
            </Story>
            <Story name={"Disabled"} background={StoryBackground::White}>
                <BaseCheckboxInput label={"Disabled Apples"} checked={toggle_checked} onchange={toggle_change} disabled={true}/>
            </Story>
            <OutlinedSecondaryButton onclick={toggle_button}>{"Toggle Checkbox"}</OutlinedSecondaryButton>
        </Stories>
    }
}
