use gloo::file::File;
use yew::prelude::*;
use yew_heroicons::size_20::solid::DocumentPlusIcon;

use crate::utils::handle_event::get_input_file_from_event;
use crate::utils::toasts::notify_err;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub on_change: Callback<Option<File>>,
    #[prop_or_default]
    pub file_name: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or_default]
    pub tip: Option<String>,
    #[prop_or_default]
    pub on_blur: Callback<()>,
    #[prop_or_default]
    pub input_ref: NodeRef,
}

#[function_component]
pub fn BaseFileInput(props: &Props) -> Html {
    let Props {
        label,
        on_change,
        file_name,
        required,
        disabled,
        error,
        tip,
        on_blur,
        input_ref,
    } = props.clone();

    let is_focused_handle = use_state(|| false);
    let is_focused = *is_focused_handle;

    let handle_focus = {
        let is_focused_handle = is_focused_handle.clone();
        Callback::from(move |_| {
            is_focused_handle.set(true);
        })
    };
    let handle_blur = on_blur.reform(move |_| {
        is_focused_handle.set(false);
    });

    let handle_click = Callback::from(move |e: MouseEvent| {
        if disabled {
            e.prevent_default();
        }
    });

    let handle_change = {
        Callback::from(move |e: Event| {
            let file = match get_input_file_from_event(e) {
                Ok(ok) => ok,
                Err(err) => return notify_err(err),
            };
            on_change.emit(Some(file));
        })
    };

    let text_label_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        if disabled {
            "text-washed-out-secondary"
        } else if is_focused {
            "text-primary"
        } else if error.is_some() {
            "text-danger"
        } else {
            "text-secondary"
        },
        "peer-focus:text-primary",
    );

    let input_wrapper_class = classes!(
        "flex",
        "justify-between",
        "gap-1",
        "px-2",
        "border-solid",
        "border-0",
        "border-b",
        "font-sans",
        "font-normal",
        "text-base",
        "leading-7",
        if disabled {
            "text-washed-out-secondary"
        } else {
            "text-secondary"
        },
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

    let label_wrapper_class = classes!(
        "flex",
        "flex-col",
        if disabled {
            "cursor-not-allowed"
        } else {
            "cursor-pointer"
        }
    );

    let error_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        "text-danger",
        "first-letter:uppercase"
    );

    let file_name_class = "whitespace-nowrap scrollbar-hide custom-horizontal-scrollbar";

    let name = label.to_lowercase().replace(' ', "-");
    let id = format!("{name}-file-input");
    let label = if required { format!("{label}*") } else { label };
    let err_id = error.as_ref().map(|_| format!("{id}-error"));

    html! {
        <div class="flex flex-col">
            <label for={id.clone()} class={label_wrapper_class} onclick={handle_click}>
                <span class={text_label_class}>{label}</span>
                <div class={input_wrapper_class}>
                    <span aria-hidden="true" class={file_name_class}>
                        {file_name.unwrap_or_else(|| "No file selected".to_string())}
                    </span>
                    <DocumentPlusIcon class="w-5 min-w-[1.25rem]" />
                    <input
                        {id}
                        class="sr-only file:border-0 file:bg-transparent"
                        type={"file"}
                        onchange={handle_change}
                        aria-errormessage={err_id.clone()}
                        {disabled}
                        onfocus={handle_focus}
                        onblur={handle_blur}
                        ref={input_ref}
                    />
                </div>
            </label>
            if let Some(error) = error {
                <div id={err_id.clone()} class={classes!(error_class)} >{error}</div>
            } else if let Some(tip) = tip {
                <small>{tip}</small>
            }
        </div>
    }
}
