use std::ops::Deref;

use clone_on_capture::clone_on_capture;
use yew::prelude::*;

use crate::components::buttons::outlined_secondary_button::OutlinedSecondaryButton;
use crate::components::form::base::base_filterable_nullable_select::BaseFilterableNullableSelect;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[clone_on_capture]
#[function_component]
pub fn BaseFilterableNullableSelectStory() -> Html {
    let value_handle = use_state(Option::<String>::default);
    let value = value_handle.deref().clone();

    let on_select = move |s| {
        log::info!("on_select({s:?})");
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
                <BaseFilterableNullableSelect
                    options={options.clone()}
                    label="Animal"
                    on_select={on_select.clone()}
                    value={value.clone()}
                />
            </Story>
            <Story name={"Required"} background={StoryBackground::White}>
                <BaseFilterableNullableSelect
                    options={options.clone()}
                    label="Required Animal"
                    value={value.clone()}
                    on_select={on_select.clone()}
                    required={true}
                />
            </Story>
            <Story name={"Disabled"} background={StoryBackground::White}>
                <BaseFilterableNullableSelect
                    options={options.clone()}
                    label="Disabled Animal"
                    value={value.clone()}
                    on_select={on_select.clone()}
                    disabled={true}
                />
            </Story>
            <Story name={"With Tip"} background={StoryBackground::White}>
                <BaseFilterableNullableSelect
                    options={options.clone()}
                    label="Tipped Animal"
                    value={value.clone()}
                    on_select={on_select.clone()}
                    tip={"This is a tip"}
                />
            </Story>
            <Story name={"With Error"} background={StoryBackground::White}>
                <BaseFilterableNullableSelect
                    options={options}
                    label="Invalid Animal"
                    value={value.clone()}
                    on_select={on_select.clone()}
                    error={"Must contain only ASCII letters"}
                />
            </Story>
            <Story name={"No options"} background={StoryBackground::White}>
                <BaseFilterableNullableSelect
                    options={vec![]}
                    label="No Animals"
                    value={value.clone()}
                    on_select={on_select.clone()}
                />
            </Story>
            <Story name={"With lots of options and a long option"} background={StoryBackground::White}>
                <BaseFilterableNullableSelect
                    options={big_options}
                    label="With lots of options and a long option Animal"
                    value={value}
                    on_select={on_select}
                />
            </Story>
        <OutlinedSecondaryButton onclick={Callback::from(move |_| value_handle.set(Some("Dog".to_string())))}>
            {"Override value programmatically to Dog"}
        </OutlinedSecondaryButton>
        </Stories>
    }
}
