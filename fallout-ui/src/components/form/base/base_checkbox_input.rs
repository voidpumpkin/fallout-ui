use web_sys::HtmlInputElement;
use yew::html;
use yew::prelude::*;
use yew::Callback;

use crate::utils::handle_event::get_checked_value_from_event;
use crate::utils::toasts::notify_err;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onchange: Callback<(bool, Event)>,
    #[prop_or_default]
    pub input_ref: NodeRef,
    // input specific attributes
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub data_qa: String,
    // classes props
    #[prop_or_default]
    pub label_class: Classes,
    #[prop_or_default]
    pub input_class: Classes,
}

#[function_component]
pub fn BaseCheckboxInput(props: &Props) -> Html {
    let Props {
        label,
        checked,
        disabled,
        onchange,
        input_ref,
        name,
        id,
        data_qa,
        label_class,
        input_class,
    } = props.clone();

    let internal_ref = use_node_ref();

    let input_ref = match input_ref.get() {
        Some(_) => input_ref,
        None => internal_ref,
    };

    let name = name.unwrap_or_else(|| label.clone().to_lowercase().replace(' ', "-"));
    let id = id.unwrap_or_else(|| format!("{name}-input"));

    let onchange = {
        Callback::from(
            move |e: Event| match get_checked_value_from_event(e.clone()) {
                Ok(checked) => {
                    onchange.emit((checked, e));
                }
                Err(err) => {
                    notify_err(err);
                }
            },
        )
    };

    let onchange = if disabled { Callback::noop() } else { onchange };

    // sync html node with the value
    use_effect({
        let input_ref = input_ref.clone();
        move || {
            if let Some(element) = input_ref.cast::<HtmlInputElement>() {
                element.set_checked(checked);
            }
            || {}
        }
    });

    let label_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        "inline-block",
        "align-text-middle",
        if disabled {
            classes!("text-washed-out-secondary", "cursor-not-allowed")
        } else {
            classes!("text-secondary", "cursor-pointer")
        },
        label_class
    );

    let input_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        "h-4",
        "w-4",
        "px-2",
        if disabled {
            classes!("border-washed-out-secondary", "cursor-not-allowed")
        } else {
            classes!("border-secondary", "cursor-pointer")
        },
        "align-middle",
        "float-left",
        "mr-2",
        "accent-primary",
        input_class
    );

    html! {
        <label class={label_class}>
            <input
                class={input_class}
                {id}
                {name}
                type={"checkbox"}
                ref={input_ref}
                {onchange}
                {disabled}
                {checked}
                data-qa={format!("{data_qa}-{label}-checkbox")}
            />
            {label}
        </label>
    }
}
