use crate::utils::validation::validation_error_from_string;
use validator::Validate;
use validator::ValidationError;
use yew::prelude::*;

use crate::components::form::form::Form;
use crate::components::form::form_submit_button::FormSubmitButton;
use crate::components::form::multiple_words_input::MultipleValues;
use crate::components::form::multiple_words_input::MultipleWordsInput;
use crate::components::form::multiple_words_input::MultipleWordsInputError;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::utils::form_schema_handle::FormSchemaHandle;
use crate::utils::toasts::notify_err;
use crate::utils::web_error::web_err_logic;
use crate::{self as fallout_ui};
use fallout_ui_macro::FormFields;

#[derive(Debug, Validate, Default, Clone, PartialEq, Eq, FormFields)]
pub struct FormSchema {
    #[validate(custom(function = "validate_minion_names"))]
    minion_names: MultipleValues,
}

#[derive(Debug, Validate, Default, Clone, PartialEq, Eq, FormFields)]
pub struct FormSchema2 {
    #[validate(custom(function = "validate_bad_minions"))]
    bad_minions: MultipleValues,
}

fn validate_minion_names(minion_names: &MultipleValues) -> Result<(), ValidationError> {
    let field_errors = minion_names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            (name.len() < 2 && minion_names.len() - 1 != i).then(|| "min length 2".to_string())
        })
        .collect::<Vec<Option<String>>>();

    if field_errors.iter().any(|err| err.is_some()) {
        let errs_json: String = match serde_json::to_string(&MultipleWordsInputError {
            error: None,
            field_errors,
        }) {
            Ok(errs_json) => errs_json,
            Err(serde_err) => {
                notify_err(web_err_logic(serde_err));
                String::default()
            }
        };

        Err(validation_error_from_string(errs_json))
    } else {
        Ok(())
    }
}

fn validate_bad_minions(bad_minions: &MultipleValues) -> Result<(), ValidationError> {
    let mut err = MultipleWordsInputError::default_from_field_values(bad_minions);
    err.error = Some("Dont eat bananas".to_string());

    let errs_json: String = match serde_json::to_string(&err) {
        Ok(errs_json) => errs_json,
        Err(serde_err) => {
            notify_err(web_err_logic(serde_err));
            String::default()
        }
    };

    Err(validation_error_from_string(errs_json))
}

#[function_component]
pub fn MultipleWordsInputStory() -> Html {
    let (form_handle, fields_control_props) =
        use_form_schema(|| FormSchemaHandle::new(FormSchema::default()));
    let (form_handle_2, fields_control_props_2) =
        use_form_schema_2(|| FormSchemaHandle::new(FormSchema2::default()));

    use_effect_with((), move |_| {
        form_handle_2.dispatch(FormSchema2Action::TouchAll);
        || {}
    });

    let on_submit = move |_| {
        form_handle.dispatch(FormSchemaAction::TouchAll);

        if form_handle.has_errors() {
            return;
        }

        log::info!("MultipleWordsInputStory submitted {:?}", form_handle.values);
    };

    html! {
        <Stories>
            <Story name={"Plain"} background={StoryBackground::White}>
                <Form {on_submit}>
                    <MultipleWordsInput legend={"Minions"} input_label={"Name"} field_control_props={fields_control_props.minion_names}/>
                    <FormSubmitButton />
                </Form>
            </Story>
            <Story name={"Group error"} background={StoryBackground::White}>
                <Form on_submit={Callback::noop()}>
                    <MultipleWordsInput legend={"Bad Minions"} input_label={"Name"} field_control_props={fields_control_props_2.bad_minions}/>
                </Form>
            </Story>
        </Stories>
    }
}
