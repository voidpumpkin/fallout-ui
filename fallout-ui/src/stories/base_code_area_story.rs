use std::ops::Deref;

use yew::prelude::*;

use crate::components::buttons::outlined_secondary_button::OutlinedSecondaryButton;
use crate::components::form::base::base_code_area::BaseCodeArea;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::handle_oninput;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

const TEST_CODE: &str = r#"// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
}"#;

#[function_component]
pub fn BaseCodeAreaStory(props: &Props) -> Html {
    let Props {} = props.clone();

    let first_name_handle = use_state_eq(String::default);
    let first_name = first_name_handle.deref().clone();

    html! {
        <Stories class="lg:max-w-md">
            <Story name={"Plain"} background={StoryBackground::White}>
                <BaseCodeArea label={"First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} />
            </Story>
            <Story name={"Required"} background={StoryBackground::White}>
                <BaseCodeArea label={"Required First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} required={true} />
            </Story>
            <Story name={"Disabled"} background={StoryBackground::White}>
                <BaseCodeArea label={"Disabled First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} disabled={true} />
            </Story>
            <Story name={"With Tip"} background={StoryBackground::White}>
                <BaseCodeArea label={"Tipped First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)}  tip="This is a tip"/>
            </Story>
            <Story name={"With Error"} background={StoryBackground::White}>
                <BaseCodeArea label={"Error First name"} value={first_name.clone()} oninput={handle_oninput!(first_name_handle)} error={"Must contain only ASCII letters"}/>
            </Story>
            <Story name={"With Error"} background={StoryBackground::White}>
                <BaseCodeArea label={"Min lines 2 and copy button"} value={first_name} oninput={handle_oninput!(first_name_handle)} min_lines={2} show_copy_cta={true} />
            </Story>
            <OutlinedSecondaryButton onclick={Callback::from(move |_| first_name_handle.set(TEST_CODE.to_string()))}>
                {"Override value programmatically"}
            </OutlinedSecondaryButton>
        </Stories>
    }
}
