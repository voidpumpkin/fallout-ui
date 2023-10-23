use yew::prelude::*;
use yew_heroicons::size_24::solid::ArrowRightIcon;

use crate::components::callouts::callout_danger::CalloutDanger;
use crate::components::callouts::callout_info::CalloutInfo;
use crate::components::callouts::callout_success::CalloutSuccess;
use crate::components::callouts::callout_warning::CalloutWarning;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[function_component]
pub fn CalloutStory() -> Html {
    html! {
        <Stories>
            <Story name={"Callout Info"} background={StoryBackground::Light}>
                <CalloutInfo>
                    {"Info should be used for communicating tips/information for the user"}
                </CalloutInfo>
            </Story>
            <Story name={"Callout Success"} background={StoryBackground::Light}>
                <CalloutSuccess>
                    {"Success should be used for communicating success cases for the user"}
                </CalloutSuccess>
            </Story>
            <Story name={"Callout Warning"} background={StoryBackground::Light}>
                <CalloutWarning>
                    {"Warning should be used for cases when user can fix it themselves or there are clear steps to be taken."}
                </CalloutWarning>
            </Story>
            <Story name={"Callout Danger"} background={StoryBackground::Light}>
                <CalloutDanger>
                    {"Danger callouts can be used for situations where there is no clear step to be taken but just report a bug or do nothing, ex:"}
                    <span class="inline-flex items-baseline">
                        <ArrowRightIcon class="fill-transparent stroke-black self-center w-5 h-5 mx-1 relative top-1"/>
                    </span>
                    {"We ran into a problem, try refreshing the page or contact our support."}
                    <span class="inline-flex items-baseline">
                        <ArrowRightIcon class="fill-transparent stroke-black self-center w-5 h-5 mx-1 relative top-1"/>
                    </span>
                    {"This account has been frozen due to an internal issue, please come back later."}
                </CalloutDanger>
            </Story>
        </Stories>
    }
}
