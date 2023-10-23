use yew::prelude::*;

use crate::components::buttons::primary_button::PrimaryButton;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::utils::toasts::notify_error;
use crate::utils::toasts::notify_info;
use crate::utils::toasts::notify_success;
use crate::utils::toasts::notify_warning;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn ToastsStory(props: &Props) -> Html {
    let Props {} = props.clone();

    html! {
        <Stories>
            <Story name={"Info"} background={StoryBackground::Light}>
                <PrimaryButton onclick={|_| notify_info("Info")}>{"Trigger Info toast"}</PrimaryButton>
            </Story>
            <Story name={"Success"} background={StoryBackground::Light}>
                <PrimaryButton onclick={|_| notify_success("Success")}>{"Trigger Success toast"}</PrimaryButton>
            </Story>
            <Story name={"Warning"} background={StoryBackground::Light}>
                <PrimaryButton onclick={|_| notify_warning("Warning")}>{"Trigger Warning toast"}</PrimaryButton>
            </Story>
            <Story name={"Error"} background={StoryBackground::Light}>
                <PrimaryButton onclick={|_| notify_error("Error")}>{"Trigger Error toast"}</PrimaryButton>
            </Story>
        </Stories>
    }
}
