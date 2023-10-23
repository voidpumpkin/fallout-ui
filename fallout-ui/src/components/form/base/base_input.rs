use clone_on_capture::clone_on_capture;
use yew::prelude::*;

use crate::utils::handle_event::get_input_value;
use crate::utils::handle_event::get_input_value_from_event;
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
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub tip: Option<String>,
    #[prop_or_default]
    pub decoration_left: Html,
    #[prop_or_default]
    pub decoration_right: Html,
    /// Used for cases when you need non-unique looking labels visually, but unique semantically
    /// e.g. instead of Director 1, Director 2, its just Director, Director
    #[prop_or_default]
    pub label_key: Option<String>,
    // input specific attributes
    #[prop_or("text".to_string())]
    pub input_type: String,
    #[prop_or("text".to_string())]
    pub input_mode: String,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or("off".to_string())]
    pub autocomplete: String,
    #[prop_or_default]
    pub min: Option<String>,
    #[prop_or_default]
    pub step: Option<String>,
    #[prop_or_default]
    pub max: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub placeholder: Option<String>,
    // classes props
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub label_class: Classes,
    #[prop_or_default]
    pub input_class: Classes,
    #[prop_or_default]
    pub error_class: Classes,
    #[prop_or_default]
    pub tip_class: Classes,
}

#[clone_on_capture]
#[function_component]
pub fn BaseInput(props: &Props) -> Html {
    let Props {
        label,
        input_ref,
        value,
        input_type,
        input_mode,
        onchange,
        oninput,
        name,
        id,
        autocomplete,
        container_class,
        label_class,
        input_class,
        tip,
        error,
        error_class,
        tip_class,
        required,
        disabled,
        placeholder,
        min,
        step,
        max,
        onblur,
        onfocus,
        label_key,
        decoration_left,
        decoration_right,
        readonly,
    } = props.clone();

    let disabled = readonly || disabled;

    let is_focused_handle = use_state(bool::default);
    let is_focused = *is_focused_handle;

    let mut name = name.unwrap_or_else(|| label.clone().to_lowercase().replace(' ', "-"));
    if let Some(label_key) = label_key.as_ref() {
        name += format!("-{label_key}").as_str();
    }
    let id = id.unwrap_or_else(|| format!("{name}-input"));

    let err_id = error.as_ref().map(|_| format!("{id}-error"));

    let handle_focus = onfocus.reform(move |_| is_focused_handle.set(true));
    let handle_blur = onblur.reform(move |_| is_focused_handle.set(false));

    let oninput = oninput.reform(move |e: InputEvent| match get_input_value(e.clone()) {
        Ok(value) => (value, e),
        Err(err) => {
            notify_err(err);
            (value.clone(), e)
        }
    });

    let onchange = onchange.reform(
        move |e: Event| match get_input_value_from_event(e.clone()) {
            Ok(value) => (value, e),
            Err(err) => {
                notify_err(err);
                (value.clone(), e)
            }
        },
    );

    let label_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        if readonly {
            "text-secondary"
        } else if disabled {
            "text-washed-out-secondary"
        } else if is_focused {
            "text-primary"
        } else if error.is_some() {
            "text-danger"
        } else {
            "text-secondary"
        },
        label_class
    );
    let input_class = classes!(
        "font-sans",
        "font-normal",
        "text-base",
        "bg-transparent",
        "w-full",
        if readonly {
            "text-secondary"
        } else if disabled {
            "text-washed-out-secondary"
        } else {
            "text-secondary"
        },
        "leading-7",
        "px-0",
        "border-0",
        "focus-visible:outline-none",
        "grow",
        input_class
    );
    let input_wrapper_class = classes!(
        "flex",
        "px-2",
        "gap-1",
        "border-solid",
        "border-0",
        "border-b",
        "justify-center",
        "items-center",
        "box-border",
        if disabled {
            "border-washed-out-secondary"
        } else if is_focused {
            "border-primary"
        } else if error.is_some() {
            "border-danger"
        } else {
            "border-secondary"
        }
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

    html! {
        <div class={classes!("flex","flex-col","w-full",container_class)}>
            <label class={label_class} for={id.clone()}>
                {label}
                if let Some(label_key) = label_key {
                    <span class="sr-only">{" "}{label_key}</span>
                }
                {required.then_some("*").unwrap_or_default()}
            </label>
            <div class={input_wrapper_class}>
                {decoration_left}
                <input
                    class={input_class}
                    id={id.clone()}
                    {name}
                    {autocomplete}
                    type={input_type}
                    ref={input_ref}
                    inputmode={input_mode}
                    {value}
                    {oninput}
                    {onchange}
                    onfocus={handle_focus}
                    onblur={handle_blur}
                    aria-invalid={invalid.to_string()}
                    aria-errormessage={err_id.clone()}
                    {required}
                    {disabled}
                    {placeholder}
                    {min}
                    {step}
                    {max}
                />
                {decoration_right}
            </div>
            if let Some(error) = error {
                <div id={err_id.clone()} class={classes!(error_class)}>{error}</div>
            } else if let Some(tip) = tip {
                <small class={tip_class}>{tip}</small>
            }
        </div>
    }
}
