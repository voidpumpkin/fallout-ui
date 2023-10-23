use std::ops::Deref;

use yew::prelude::*;
use yew_heroicons::size_20::solid::BeakerIcon;

use crate::components::buttons::icon_button_for_input_decoration::IconButtonForInputDecoration;
use crate::components::buttons::outlined_secondary_button::OutlinedSecondaryButton;
use crate::components::form::base::base_input::BaseInput;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::handle_oninput;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn BaseInputStory(props: &Props) -> Html {
    let Props {} = props.clone();

    let first_name_handle = use_state_eq(String::default);
    let first_name = first_name_handle.deref().clone();

    let decoration = html! {
        <IconButtonForInputDecoration>
            <BeakerIcon />
        </IconButtonForInputDecoration>
    };

    html! {
        <Stories class="lg:max-w-md">
            <Story name={"Plain"} background={StoryBackground::White}>
                <BaseInput label={"First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} />
            </Story>
            <Story name={"Required"} background={StoryBackground::White}>
                <BaseInput label={"Required First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} required={true} />
            </Story>
            <Story name={"Disabled"} background={StoryBackground::White}>
                <BaseInput label={"Disabled First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} disabled={true} />
            </Story>
            <Story name={"With tip"} background={StoryBackground::White}>
                <BaseInput label={"Tipped First name"} value={first_name.clone()} tip="This is a tip" oninput={handle_oninput!(first_name_handle)} />
            </Story>
            <Story name={"With Error"} background={StoryBackground::White}>
                <BaseInput label={"Invalid First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} error={"Must contain only ASCII letters"}/>
            </Story>
            <Story name={"Decoration Left"} background={StoryBackground::White}>
                <BaseInput label={"Decoration Left"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} decoration_left={decoration.clone()}/>
            </Story>
            <Story name={"Decoration Right"} background={StoryBackground::White}>
                <BaseInput label={"Decoration Right"} value={first_name} oninput={handle_oninput!(first_name_handle)} decoration_right={decoration}/>
            </Story>
            <OutlinedSecondaryButton onclick={move |_| first_name_handle.set("Dog".to_string())}>
                {"Override value programmatically to Dog"}
            </OutlinedSecondaryButton>
        </Stories>
    }
}
