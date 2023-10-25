use std::ops::Deref;

use clone_on_capture::clone_on_capture;
use yew::prelude::*;

use crate::components::buttons::outlined_secondary_button::OutlinedSecondaryButton;
use crate::components::form::base::base_multi_select::BaseMultiSelect;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[clone_on_capture]
#[function_component]
pub fn BaseMultiSelectStory() -> Html {
    let value_handle = use_state(Vec::<String>::default);
    let value = value_handle.deref().clone();

    let on_change = move |s| {
        log::info!("on_change({s:?})");
        value_handle.set(s);
    };

    let options = vec![
        "Cat".to_string(),
        "Dolphin".to_string(),
        "Dog".to_string(),
        "Seal".to_string(),
    ];

    let mut big_options = vec![];
    big_options.append(&mut options.clone());
    big_options.push(vec!["Seagull"; 50].join(" "));
    big_options.append(&mut (0..100).map(|i| format!("Cat{i}")).collect());

    html! {
        <Stories class="lg:max-w-md">
            <Story name={"Plain"} background={StoryBackground::White}>
                <BaseMultiSelect
                    options={options.clone()}
                    label="Animals"
                    on_change={on_change.clone()}
                    value={value.clone()}
                />
            </Story>
            <Story name={"Required"} background={StoryBackground::White}>
                <BaseMultiSelect
                    options={options.clone()}
                    label="Required Animals"
                    value={value.clone()}
                    on_change={on_change.clone()}
                    required={true}
                />
            </Story>
            <Story name={"Disabled"} background={StoryBackground::White}>
                <BaseMultiSelect
                    options={options.clone()}
                    label="Disabled Animals"
                    value={value.clone()}
                    on_change={on_change.clone()}
                    disabled={true}
                />
            </Story>
            <Story name={"With Tip"} background={StoryBackground::White}>
                <BaseMultiSelect
                    options={options.clone()}
                    label="Tipped Animals"
                    value={value.clone()}
                    on_change={on_change.clone()}
                    tip={"This is a tip"}
                />
            </Story>
            <Story name={"With Error"} background={StoryBackground::White}>
                <BaseMultiSelect
                    options={options}
                    label="Invalid Animals"
                    value={value.clone()}
                    on_change={on_change.clone()}
                    error={"Must contain only ASCII letters"}
                />
            </Story>
            <Story name={"No options"} background={StoryBackground::White}>
                <BaseMultiSelect
                    options={vec![]}
                    label="No Animals"
                    value={value.clone()}
                    on_change={on_change.clone()}
                />
            </Story>
            <Story name={"With lots of options and a long option"} background={StoryBackground::White}>
                <BaseMultiSelect
                    options={big_options}
                    label="With lots of options and a long option Animals"
                    value={value}
                    on_change={on_change}
                />
            </Story>
            <OutlinedSecondaryButton onclick={move |_| value_handle.set(vec!["Dog".to_string()])}>
                {"Override value programmatically to Dog"}
            </OutlinedSecondaryButton>
        </Stories>
    }
}
