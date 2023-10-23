use yew::prelude::*;

use crate::utils::handle_event::get_text_area_value;
use crate::utils::handle_event::get_text_area_value_from_event;
use crate::utils::toasts::notify_err;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    // core props
    pub label: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub oninput: Callback<(String, InputEvent)>,
    #[prop_or_default]
    pub onchange: Callback<(String, Event)>,
    #[prop_or_default]
    pub onblur: Callback<()>,
    #[prop_or_default]
    pub onfocus: Callback<()>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or_default]
    pub textarea_ref: NodeRef,
    #[prop_or_default]
    pub tip: Option<String>,
    /// Used for cases when you need non-unique looking labels visually, but unique semantically
    /// e.g. instead of Director 1, Director 2, its just Director, Director
    #[prop_or_default]
    pub label_key: Option<String>,
    // textarea specific attributes
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or("off".to_string())]
    pub autocomplete: String,
    #[prop_or_default]
    pub disabled: bool,
    // classes props
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub label_class: Classes,
    #[prop_or_default]
    pub textarea_class: Classes,
    #[prop_or_default]
    pub error_class: Classes,
    #[prop_or_default]
    pub tip_class: Classes,
}

#[function_component]
pub fn BaseTextArea(props: &Props) -> Html {
    let Props {
        label,
        textarea_ref,
        value,
        onchange,
        oninput,
        id,
        autocomplete,
        container_class,
        label_class,
        textarea_class,
        tip,
        error,
        error_class,
        tip_class,
        required,
        disabled,
        onblur,
        onfocus,
        label_key,
    } = props.clone();

    let mut id = id.unwrap_or_else(|| {
        format!(
            "{}-textarea",
            label.clone().to_lowercase().replace(' ', "-")
        )
    });
    if let Some(label_key) = label_key.as_ref() {
        id += format!("-{label_key}").as_str();
    }

    let err_id = error.as_ref().map(|_| format!("{id}-error"));

    let oninput = {
        let value = value.clone();
        oninput.reform(move |e: InputEvent| match get_text_area_value(e.clone()) {
            Ok(value) => (value, e),
            Err(err) => {
                notify_err(err);
                (value.clone(), e)
            }
        })
    };

    let onchange = {
        let value = value.clone();

        onchange.reform(
            move |e: Event| match get_text_area_value_from_event(e.clone()) {
                Ok(value) => (value, e),
                Err(err) => {
                    notify_err(err);
                    (value.clone(), e)
                }
            },
        )
    };

    let label_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        if disabled {
            "text-washed-out-secondary"
        } else if error.is_some() {
            "text-danger"
        } else {
            "text-secondary"
        },
        "peer-focus:text-primary",
        label_class
    );
    let textarea_class = classes!(
        "font-sans",
        "font-normal",
        "text-base",
        if disabled {
            "text-washed-out-secondary"
        } else {
            "text-secondary"
        },
        "px-2",
        "border-solid",
        "border-0",
        "border-b",
        "resize-none",
        if disabled {
            "border-washed-out-secondary"
        } else if error.is_some() {
            "border-danger"
        } else {
            "border-secondary"
        },
        "focus:border-primary",
        "focus-visible:outline-none",
        "peer",
        "overflow-y-hidden",
        "overflow-x-auto",
        "resize-none",
        "break-normal",
        "whitespace-pre",
        "custom-horizontal-scrollbar",
        textarea_class
    );
    let tip_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        "text-washed-out-secondary",
        tip_class
    );
    let error_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        "text-danger",
        "first-letter:uppercase",
        error_class
    );

    let invalid = error.is_some();
    // Empty ending lines are not treated as a line: "hi\n" -> 1
    let lines_count = format!("{value} ").lines().count();

    html! {
        // Children of the container div are reversed to allow for "peer" to work, check tailwind docs for more details
        <div class={classes!("flex", "flex-col-reverse", "group", container_class)}>
            if let Some(error) = error {
                <div id={err_id.clone()} class={classes!(error_class)}>{error}</div>
            } else if let Some(tip) = tip {
                <small class={tip_class}>{tip}</small>
            }
            <textarea
                class={textarea_class}
                id={id.clone()}
                {autocomplete}
                ref={textarea_ref}
                {value}
                {oninput}
                {onchange}
                onfocus={onfocus.reform(|_|{})}
                onblur={onblur.reform(|_|{})}
                aria-invalid={invalid.to_string()}
                aria-errormessage={err_id.clone()}
                {required}
                {disabled}
                style={format!("height: calc(1.5rem * {lines_count});")}
            />
            <label
                class={label_class}
                for={id.clone()}
            >
                {label}
                if let Some(label_key) = label_key {
                    <span class="sr-only">{" "}{label_key}</span>
                }
                {required.then_some("*").unwrap_or_default()}
            </label>
        </div>
    }
}
