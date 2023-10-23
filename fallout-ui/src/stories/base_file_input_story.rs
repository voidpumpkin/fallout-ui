use gloo::file::File;
use yew::prelude::*;

use crate::components::form::base::base_file_input::BaseFileInput;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[function_component]
pub fn BaseFileInputStory() -> Html {
    let value_handle = use_state(Option::<File>::default);

    let on_change = move |s| {
        log::info!("on_change({s:?})");
        value_handle.set(s);
    };

    html! {
        <Stories class="lg:max-w-md">
            <Story name={"Plain"} background={StoryBackground::White}>
                <BaseFileInput
                    label="Animal"
                    on_change={on_change.clone()}
                />
            </Story>
            <Story name={"Required"} background={StoryBackground::White}>
                <BaseFileInput
                    label="Required Animal"
                    on_change={on_change.clone()}
                    required={true}
                />
            </Story>
            <Story name={"Disabled"} background={StoryBackground::White}>
                <BaseFileInput
                    label="Disabled Animal"
                    on_change={on_change.clone()}
                    disabled={true}
                />
            </Story>
            <Story name={"With Tip"} background={StoryBackground::White}>
                <BaseFileInput
                    label="With Tip Animal"
                    on_change={on_change.clone()}
                    tip={"This is a tip"}
                />
            </Story>
            <Story name={"With Error"} background={StoryBackground::White}>
                <BaseFileInput
                    label="Invalid Animal"
                    on_change={on_change}
                    error={"This file is OUTRAGEOUS!"}
                />
            </Story>
        </Stories>
    }
}
