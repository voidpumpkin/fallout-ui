use std::ops::Deref;

use clone_on_capture::clone_on_capture;
use yew::prelude::*;

use crate::components::callouts::callout_warning::CalloutWarning;
use crate::components::form::base::base_input::BaseInput;
use crate::components::form::form::Form;
use crate::components::form::form_submit_button::FormSubmitButton;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::handle_oninput;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[clone_on_capture]
#[function_component]
pub fn FormStory(props: &Props) -> Html {
    let Props {} = props.clone();

    let first_name_handle = use_state_eq(String::default);
    let first_name = first_name_handle.deref().clone();

    let on_submit = move |_| log::info!("Submitted with first name: {}", first_name);

    html! {
        <Stories>
            <CalloutWarning>
                {"This example only highlights a low level concept, for how to make more powerful forms visit "}
                <a href={"/story/form_fields_macro"}>{"FormFieldsMacro story"}</a>
            </CalloutWarning>
            <Story name={"Form with a controlled input"} background={StoryBackground::White}>
                <Form {on_submit}>
                    <BaseInput label={"First name"} value={first_name} oninput={handle_oninput!(first_name_handle)} />
                    <FormSubmitButton />
                </Form>
            </Story>
        </Stories>
    }
}
