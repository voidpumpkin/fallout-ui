use std::fmt::Debug;
use std::ops::Deref;

use clone_on_capture::clone_on_capture;
use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_heroicons::size_20::solid::ChevronDownIcon;
use yew_heroicons::size_20::solid::ChevronUpIcon;

use crate::components::buttons::icon_button_for_input_decoration::IconButtonForInputDecoration;
use crate::components::custom_checkbox::CustomCheckbox;
use crate::utils::get_page_offset_xy;
use crate::utils::handle_event::get_input_value;
use crate::utils::toasts::notify_err;
use crate::utils::web_error::web_err_js;
use crate::utils::web_error::web_err_logic;

#[derive(Clone, Debug, PartialEq)]
enum DropDownValue<O: Clone + Debug + PartialEq + ToString + 'static> {
    All,
    Option(O),
}

fn get_default_selection_amount_text((values_len, options_len): (usize, usize)) -> String {
    format!("{values_len}/{options_len} selected")
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props<O>
where
    O: Clone + Debug + PartialEq + ToString + 'static,
{
    pub label: String,
    #[prop_or_default]
    pub value: Vec<O>,
    pub options: Vec<O>,
    pub on_change: Callback<Vec<O>>,
    #[prop_or_default]
    pub tip: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onblur: Callback<()>,
    #[prop_or(Callback::from(|(values_len, options_len, _): (usize, usize, String)| get_default_selection_amount_text((values_len, options_len))))]
    pub selection_amount_text: Callback<(usize, usize, String), String>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or(Callback::from(|option: String| -> Html { option.into() }))]
    pub option_html: Callback<String, Html>,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub hide_select_all: bool,
}

fn toggle_option<O>(mut arr: Vec<O>, option: O) -> Vec<O>
where
    O: Clone + Debug + PartialEq + ToString + 'static,
{
    if arr.contains(&option) {
        arr.into_iter().filter(|o| o != &option).collect()
    } else {
        arr.push(option);
        arr
    }
}

fn toggle_all<O>(values: Vec<O>, all_options: Vec<O>) -> Vec<O>
where
    O: Clone + Debug + PartialEq + ToString + 'static,
{
    if values.len() == all_options.len() {
        vec![]
    } else {
        all_options
    }
}

#[clone_on_capture]
#[function_component]
pub fn BaseMultiSelect<O = String>(props: &Props<O>) -> Html
where
    O: Clone + Debug + PartialEq + ToString + 'static,
{
    let Props {
        label,
        on_change,
        value,
        options,
        tip,
        required,
        disabled,
        onblur,
        error,
        option_html,
        selection_amount_text,
        container_class,
        hide_select_all,
    } = props.clone();

    let has_options = !options.is_empty();

    let disabled = disabled || !has_options;

    let all_options = options.clone();

    let selection_amount_text = use_memo(
        (value.len(), options.len(), selection_amount_text.clone()),
        move |(values_len, options_len, _)| {
            selection_amount_text.emit((
                *values_len,
                *options_len,
                get_default_selection_amount_text((*values_len, *options_len)),
            ))
        },
    );

    let input_value_handle = use_state_eq(String::default);
    let input_value = input_value_handle.deref().clone();

    let hovered_value_handle = use_state(Option::<DropDownValue<O>>::default);
    let hovered_value = hovered_value_handle.deref().clone();

    let list_coordinate_handle = use_state(<(i32, i32)>::default);
    let (list_coordinate_x, list_coordinate_y) = *list_coordinate_handle;

    let is_focused_handle = use_state(bool::default);
    let is_focused = *is_focused_handle;

    let input_ref = use_node_ref();
    let list_container_ref = use_node_ref();

    let handle_input = move |e: InputEvent| match get_input_value(e) {
        Ok(value) => {
            input_value_handle.set(value);
        }
        Err(err) => {
            notify_err(err);
        }
    };

    let handle_focus = move |_| {
        is_focused_handle.set(true);
        if let Some(element) = list_container_ref.cast::<HtmlElement>() {
            let (screen_x, screen_y) = match get_page_offset_xy() {
                Ok(ok) => ok,
                Err(err) => {
                    notify_err(err);
                    (0, 0)
                }
            };
            list_coordinate_handle.set((
                element.offset_left() - screen_x,
                element.offset_top() - screen_y,
            ));
        }
    };

    let onmouseleave = move |_| hovered_value_handle.set(None);

    let options: Vec<O> = options
        .into_iter()
        .filter(|o| {
            o.to_string()
                .to_lowercase()
                .contains(input_value.to_lowercase().as_str())
        })
        .collect();

    let onkeydown = move |e: KeyboardEvent| {
        let hovered_value = hovered_value_handle.deref().clone();

        match e.key().as_str() {
            key @ "ArrowDown" | key @ "ArrowUp" | key @ "Enter" => {
                if let Ok(e) = e.dyn_into::<Event>() {
                    e.prevent_default();
                }

                match key {
                    "ArrowDown" => {
                        let mut options_iter = vec![DropDownValue::All]
                            .into_iter()
                            .chain(options.clone().into_iter().map(DropDownValue::Option))
                            .cycle();
                        if let Some(hovered_value) = hovered_value {
                            options_iter.position(|o| o == hovered_value);
                        }

                        hovered_value_handle.set(options_iter.next());
                    }
                    "ArrowUp" => {
                        let mut options_iter = vec![DropDownValue::All]
                            .into_iter()
                            .chain(options.clone().into_iter().map(DropDownValue::Option))
                            .rev()
                            .cycle();
                        if let Some(hovered_value) = hovered_value {
                            options_iter.position(|o| o == hovered_value);
                        }

                        hovered_value_handle.set(options_iter.next());
                    }
                    "Enter" => match hovered_value {
                        Some(DropDownValue::All) => {
                            on_change.emit(toggle_all(value.clone(), all_options.clone()))
                        }
                        Some(DropDownValue::Option(option)) => {
                            on_change.emit(toggle_option(value.clone(), option))
                        }
                        None => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    };

    let mut options: Vec<(DropDownValue<O>, String)> = options
        .into_iter()
        .map(|o| {
            let option_string = o.to_string();
            (
                DropDownValue::Option(o),
                format!("multi-select-{label}-option-{option_string}"),
            )
        })
        .collect();

    if !hide_select_all && input_value.is_empty() {
        options.insert(
            0,
            (
                DropDownValue::All,
                format!("multi-select-{label}-select-all"),
            ),
        );
    }

    let handle_blur = move |e: FocusEvent| {
        let related_target_option_id = e
            .related_target()
            .and_then(|related_target| related_target.dyn_into::<Element>().ok())
            .map(|element| element.id())
            .and_then(|option_id| options.iter().find(|(_, id)| &option_id == id).cloned());

        if related_target_option_id.is_some() {
            if let Some(input_element) = input_ref.cast::<HtmlElement>() {
                gloo::timers::callback::Timeout::new(0, move || {
                    if let Err(err) = input_element.focus() {
                        notify_err(web_err_js(err));
                    }
                })
                .forget();
            }
            return;
        }

        is_focused_handle.set(false);
        hovered_value_handle.set(None);
        input_value_handle.set("".to_string());
        onblur.emit(());
    };

    let handle_arrow_click = move |_| {
        if disabled {
            return;
        }
        if let Some(element) = input_ref.cast::<HtmlElement>() {
            if element.focus().is_err() {
                notify_err(web_err_logic("Failed to focus Select Input"));
            }
        }
    };

    let label_class = classes!(
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
    );

    let input_class = classes!(
        "font-sans",
        "font-normal",
        "text-base",
        "bg-transparent",
        "w-full",
        if disabled {
            "text-washed-out-secondary"
        } else {
            "text-secondary"
        },
        "leading-7",
        "border-0",
        "focus-visible:outline-none",
        "p-0",
        "grow",
        (!is_focused).then_some("italic"),
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

    let options_class = classes!(
        "shadow-md",
        "bg-washed-out-thirdly",
        "p-1",
        "m-0",
        "z-popover",
        "absolute",
        "list-none",
        "min-w-full",
        "w-max",
        "top-0",
        "left-0",
        "box-border",
        "overflow-y-auto",
        (!is_focused).then_some("hidden")
    );

    let error_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        "text-danger",
        "first-letter:uppercase"
    );

    let options_html: Html = options
        .into_iter()
        .enumerate()
        .map(|(i, (option, option_id))| {
            let hovered = Some(&option) == hovered_value.as_ref();

            let li_class = classes!(
                "font-sans",
                "text-base",
                "cursor-pointer",
                "text-secondary",
                "border-solid",
                "border-0",
                "border-b",
                if hovered {
                    "border-primary"
                } else if !hide_select_all && i == 0 {
                    "border-secondary"
                } else {
                    "border-washed-out-thirdly"
                },
                (!hide_select_all && i == 0).then_some(classes!("font-medium", "italic")),
                hovered.then_some("text-primary"),
                "flex",
                "gap-2",
                "items-center",
            );

            let onmouseenter = {
                let option = option.clone();
                move |_| {
                    hovered_value_handle.set(Some(option.clone()));
                }
            };

            let onclick = {
                let option = option.clone();
                move |_| match option.clone() {
                    DropDownValue::All => {
                        on_change.emit(toggle_all(value.clone(), all_options.clone()))
                    }
                    DropDownValue::Option(option) => {
                        on_change.emit(toggle_option(value.clone(), option))
                    }
                }
            };

            let tabindex = (i + 1).to_string();

            let checked = match &option {
                DropDownValue::All => value.len() == all_options.len(),
                DropDownValue::Option(option) => value.contains(option),
            };

            let option_display = match &option {
                DropDownValue::All if value.len() == all_options.len() => html! {{"Unselect All"}},
                DropDownValue::All => html! {{"Select All"}},
                DropDownValue::Option(option) => option_html.emit(option.to_string()),
            };

            html_nested! {
                <li
                  key={option_id.clone()}
                  id={option_id.clone()}
                  {tabindex}
                  class={li_class.clone()}
                  {onmouseenter}
                  {onclick}
                >
                    <CustomCheckbox {checked} />
                    {option_display}
                </li>
            }
        })
        .collect();

    let name = label.to_lowercase().replace(' ', "-");
    let id = format!("{name}-input");
    let list_id = format!("{label}-options-list");
    let label = if required { format!("{label}*") } else { label };
    let err_id = error.as_ref().map(|_| format!("{id}-error"));
    let value = if is_focused {
        input_value
    } else {
        selection_amount_text.to_string()
    };

    html! {
        <div class={classes!("flex","flex-col","w-full", container_class)}>
            <label class={label_class} for={id.clone()}>{label}</label>
            <div class={input_wrapper_class}>
                <input
                    {id}
                    {name}
                    class={input_class}
                    onfocus={handle_focus}
                    onblur={handle_blur}
                    ref={input_ref}
                    {value}
                    oninput={handle_input}
                    {onkeydown}
                    aria-owns={list_id.clone()}
                    aria-errormessage={err_id.clone()}
                    {disabled}
                    autocomplete="off"
                />
                <IconButtonForInputDecoration onclick={handle_arrow_click} {disabled} flash_orange_on_click={false}>
                    if is_focused {
                        <ChevronUpIcon />
                    } else {
                        <ChevronDownIcon />
                    }
                </IconButtonForInputDecoration>
            </div>
            <div class="relative w-full" ref={list_container_ref}>
                <ul
                    id={list_id}
                    role="listbox"
                    class={options_class}
                    style={format!("max-height: calc(100vh - {list_coordinate_y}px - 1rem); max-width: calc(100vw - 1rem - {list_coordinate_x}px)")}
                    {onmouseleave}
                >
                    {options_html}
                </ul>
            </div>
            if let Some(error) = error {
                <div id={err_id.clone()} class={classes!(error_class)} >{error}</div>
            } else if let Some(tip) = tip {
                <small>{tip}</small>
            }
        </div>
    }
}
