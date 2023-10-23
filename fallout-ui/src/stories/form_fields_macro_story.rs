use validator::Validate;
use yew::prelude::*;

use crate::components::callouts::callout_info::CalloutInfo;
use crate::components::form::form::Form;
use crate::components::form::form_submit_button::FormSubmitButton;
use crate::components::form::multiple_words_input::MultipleValues;
use crate::components::form::multiple_words_input::MultipleWordsInput;
use crate::components::form::text_input::TextInput;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::utils::form_schema_handle::FormSchemaHandle;
use crate::utils::toasts::notify_success;
use crate::{self as fallout_ui};
use fallout_ui_macro::FormFields;

#[function_component]
pub fn FormFieldsMacroStory() -> Html {
    html! {
        <Stories>
            <CalloutInfo>{"To find out on how to use the macro please visit "}{file!()}{" file"}</CalloutInfo>
            <Story name={"Client side FormFields functionality"} background={StoryBackground::White}>
                <ClientSideFormFieldsFunctionality />
            </Story>
        </Stories>
    }
}

/// 1. Create a struct that would represent a schema for the form
/// Tip: You can inspect the generated code by using your IDEs expand macro functionality
#[derive(Debug, Validate, Default, Clone, PartialEq, Eq, FormFields)]
pub struct FormSchema {
    #[validate(length(min = 1, message = "This field is required"))]
    text: String,

    /// The field type is not limited to strings,
    /// though it is semi-limited by the Yew component you will be mapping it to.
    directors: MultipleValues,
}

/// The macro generates a few invisible structs, an enum and a hook
/// The name always depends on the schema struct name (In this case [FormSchema])
/// Simpler ones are:
/// - [FormSchemaField] enum, that can be used instead of strings that match the field name of the schema
/// - [FormSchemaFieldsControlProps] struct, for holding the cashed control props
///
/// State is managed with a [yew::prelude::Reducer] and [yew::prelude::use_reducer], instead of [yew::prelude::use_state]
/// What is a reducer in a few words?
/// You dispatch a action (message) that is received by the reducer and it reduces the state based on the action
/// Thus the macro generates also:
/// - [FormSchemaAction] enum for these actions/messages
/// - [FormSchemaFieldSetValuePayload] enum used to pass a key - value pair to the set value action
#[function_component]
pub fn ClientSideFormFieldsFunctionality() -> Html {
    // Here the hook exists, so that it would store the values (schema struct)
    // But they are wrapped in a FormSchemaHandle so it would also hold errors and other useful values
    // The hook also cashes the control_props for every input, to prevent unnecessary re-renders
    let (form_handle, fields_control_props) = use_form_schema(|| {
        FormSchemaHandle::new(FormSchema {
            ..Default::default()
        })
    });

    // All fields eventually map with their designated inputs from [legacy_ui::components::form]
    // Mapping happens via field_control_props
    // This components will also dictate what type you need to you use in the schema

    let on_submit = Callback::from(move |_| {
        // On submit you have to touch all fields
        // "Touched" means that this field was touched and error can be displayed now
        // All fields can start off by being invalid, but visually no error will be shown for better UX
        form_handle.dispatch(FormSchemaAction::TouchAll);

        // Do not submit if there are errors
        if form_handle.has_errors() {
            return;
        }

        notify_success(
            format!(
                "ClientSideFormFieldsFunctionality submitted {:#?}",
                form_handle.values
            )
            .as_str(),
        );
    });

    html! {
        <>
        <Form {on_submit}>
            <TextInput label={"Text"} required={true} field_control_props={fields_control_props.text}/>
            <MultipleWordsInput legend={"Directors"} input_label={"Director"} field_control_props={fields_control_props.directors}/>
            <FormSubmitButton />
        </Form>
        </>
    }
}
