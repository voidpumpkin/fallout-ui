use std::ops::Deref;

use yew::prelude::*;

use crate::components::buttons::outlined_secondary_button::OutlinedSecondaryButton;
use crate::components::form::base::base_text_area::BaseTextArea;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::handle_oninput;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn BaseTextAreaStory(props: &Props) -> Html {
    let Props {} = props.clone();

    let first_name_handle = use_state_eq(String::default);
    let first_name = first_name_handle.deref().clone();

    html! {
        <Stories class="lg:max-w-md">
            <Story name={"Plain"} background={StoryBackground::White}>
                <BaseTextArea label={"First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} />
            </Story>
            <Story name={"Required"} background={StoryBackground::White}>
                <BaseTextArea label={"Required First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} required={true} />
            </Story>
            <Story name={"Disabled"} background={StoryBackground::White}>
                <BaseTextArea label={"Disabled First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} disabled={true} />
            </Story>
            <Story name={"With Tip"} background={StoryBackground::White}>
                <BaseTextArea label={"Tipped First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} tip="This is a tip" />
            </Story>
            <Story name={"With Error"} background={StoryBackground::White}>
                <BaseTextArea label={"Invalid First name"} value={first_name} oninput={handle_oninput!(first_name_handle)} error={"Must contain only ASCII letters"}/>
            </Story>
        <OutlinedSecondaryButton onclick={Callback::from(move |_| first_name_handle.set("Bob".to_string()))}>
            {"Override value programmatically to Bob"}
        </OutlinedSecondaryButton>
        </Stories>
    }
}
