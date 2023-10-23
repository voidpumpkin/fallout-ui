use yew::prelude::*;

use crate::components::callouts::callout_info::CalloutInfo;
use crate::components::divider::Divider;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn DividerStory(props: &Props) -> Html {
    let Props {} = props.clone();
    html! {
        <Stories>
            <CalloutInfo>{"Can be colored using border color"}</CalloutInfo>
            <Story name={"White"} background={StoryBackground::Dark}>
                <Divider class="border-white"/>
            </Story>
            <Story name={"Dark Brown"} background={StoryBackground::Light}>
                <Divider class="border-secondary"/>
            </Story>
        </Stories>
    }
}
