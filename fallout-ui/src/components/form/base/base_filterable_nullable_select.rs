use std::fmt::Debug;
use std::iter::Cycle;
use std::iter::Rev;
use std::ops::Deref;
use std::vec::IntoIter;

use wasm_bindgen::JsCast;
use web_sys::Element;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_heroicons::size_20::solid::ChevronDownIcon;
use yew_heroicons::size_20::solid::ChevronUpIcon;

use crate::components::buttons::icon_button_for_input_decoration::IconButtonForInputDecoration;
use crate::utils::get_page_offset_xy;
use crate::utils::handle_event::get_input_value;
use crate::utils::toasts::notify_err;
use crate::utils::web_error::web_err_js;
use crate::utils::web_error::web_err_logic;

#[derive(Clone, PartialEq, Properties)]
pub struct Props<O: Clone + Debug + PartialEq + ToString + 'static> {
    pub label: String,
    /// Used for cases when you need non-unique looking labels visually, but unique semantically
    /// e.g. instead of Director 1, Director 2, its just Director, Director
    #[prop_or_default]
    pub label_key: Option<String>,
    #[prop_or_default]
    pub value: Option<O>,
    pub options: Vec<O>,
    pub on_select: Callback<Option<O>>,
    #[prop_or_default]
    pub tip: Option<String>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onblur: Callback<()>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or(Callback::from(|option: String| -> Html { option.into() }))]
    pub option_html: Callback<String, Html>,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub decoration_right: Html,
}

#[function_component]
pub fn BaseFilterableNullableSelect<O: Clone + Debug + PartialEq + ToString + 'static = String>(
    props: &Props<O>,
) -> Html {
    let Props {
        label,
        on_select,
        value,
        options,
        tip,
        required,
        disabled,
        onblur,
        error,
        option_html,
        label_key,
        container_class,
        decoration_right,
    } = props.clone();

    let has_options = !options.is_empty();

    let disabled = disabled || !has_options;
    let placeholder = (!has_options).then_some("No selection options available");

    let input_value_handle = use_state_eq({
        let value = value.clone();
        move || value.map(|o| o.to_string()).unwrap_or_default()
    });
    let input_value = input_value_handle.deref().clone();

    let input_value_on_list_close_handle = use_state({
        let value = value.clone();
        move || value.map(|o| o.to_string())
    });
    let input_value_on_list_close = input_value_on_list_close_handle.deref().clone();

    use_effect_with(value.clone(), {
        let input_value_on_list_close_handle = input_value_on_list_close_handle.clone();
        let input_value_handle = input_value_handle.clone();
        move |_| {
            match value {
                Some(value) => {
                    let value_string = value.to_string();
                    if &value_string != input_value_handle.deref() {
                        input_value_handle.set(value_string.clone());
                        input_value_on_list_close_handle.set(Some(value_string));
                    }
                }
                None => {
                    input_value_handle.set(String::default());
                    input_value_on_list_close_handle.set(None);
                }
            }

            || {}
        }
    });

    let hovered_value_handle = use_state(Option::<O>::default);
    let hovered_value = hovered_value_handle.deref().clone();

    let list_coordinate_handle = use_state(<(i32, i32)>::default);
    let (list_coordinate_x, list_coordinate_y) = *list_coordinate_handle;

    let is_focused_handle = use_state(bool::default);
    let is_focused = *is_focused_handle;

    let show_list_handle = use_state(|| false);
    let show_list = *show_list_handle;

    let input_ref = use_node_ref();
    let list_container_ref = use_node_ref();

    let options: Vec<O> = options
        .into_iter()
        .filter(|o| {
            if input_value == input_value_on_list_close.clone().unwrap_or_default() {
                return true;
            }
            o.to_string()
                .to_lowercase()
                .contains(input_value.to_lowercase().as_str())
        })
        .collect();

    let handle_input = {
        let input_value_handle = input_value_handle.clone();
        let input_value_on_list_close_handle = input_value_on_list_close_handle.clone();
        let show_list_handle = show_list_handle.clone();
        let hovered_value_handle = hovered_value_handle.clone();
        let option_strings: Vec<_> = options.clone().into_iter().map(|o| o.to_string()).collect();

        Callback::from(move |e: InputEvent| {
            show_list_handle.set(true);
            match get_input_value(e) {
                Ok(value) => {
                    if !option_strings.contains(&value) {
                        input_value_on_list_close_handle.set(None);
                    };
                    input_value_handle.set(value);
                    hovered_value_handle.set(None);
                }
                Err(err) => {
                    notify_err(err);
                }
            }
        })
    };

    let handle_focus = {
        let is_focused_handle = is_focused_handle.clone();
        let list_container_ref = list_container_ref.clone();

        Callback::from(move |_| {
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
        })
    };

    let handle_input_click = {
        let show_list_handle = show_list_handle.clone();

        Callback::from(move |_| {
            if !disabled {
                show_list_handle.set(true);
            }
        })
    };

    let onmouseleave = {
        let hovered_value_handle = hovered_value_handle.clone();

        Callback::from(move |_| {
            hovered_value_handle.set(None);
        })
    };

    let onkeydown = {
        let options = options.clone();
        let hovered_value_handle = hovered_value_handle.clone();
        let input_value_handle = input_value_handle.clone();
        let input_value_on_list_close_handle = input_value_on_list_close_handle.clone();
        let show_list_handle = show_list_handle.clone();
        Callback::from(move |e: KeyboardEvent| {
            let hovered_value = hovered_value_handle.deref().clone();

            match e.key().as_str() {
                key @ "ArrowDown" | key @ "ArrowUp" | key @ "Enter" => {
                    if let Ok(e) = e.dyn_into::<Event>() {
                        e.prevent_default();
                    }

                    match key {
                        "ArrowDown" => {
                            if !*show_list_handle {
                                return show_list_handle.set(true);
                            }

                            let mut options_iter: Cycle<IntoIter<_>> =
                                options.clone().into_iter().cycle();
                            if let Some(hovered_value) = hovered_value {
                                options_iter.position(|o| o == hovered_value);
                            }

                            hovered_value_handle.set(options_iter.next());
                        }
                        "ArrowUp" => {
                            if !*show_list_handle {
                                return show_list_handle.set(true);
                            }

                            let mut options_iter: Cycle<Rev<IntoIter<_>>> =
                                options.clone().into_iter().rev().cycle();
                            if let Some(hovered_value) = hovered_value {
                                options_iter.position(|o| o == hovered_value);
                            }

                            hovered_value_handle.set(options_iter.next());
                        }
                        "Enter" => {
                            show_list_handle.set(false);
                            input_value_on_list_close_handle
                                .set(hovered_value.as_ref().map(|o| o.to_string()));
                            if let Some(hovered_value) = hovered_value {
                                input_value_handle.set(hovered_value.to_string());
                                hovered_value_handle.set(None);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
    };

    let options: Vec<(O, String)> = options
        .into_iter()
        .map(|o| {
            let option_string = o.to_string();
            (
                o,
                format!("filterable-select-{label}-option-{option_string}"),
            )
        })
        .collect();

    let handle_blur = {
        let options = options.clone();
        let input_value = input_value.clone();
        let show_list_handle = show_list_handle.clone();
        let hovered_value_handle = hovered_value_handle.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |e: FocusEvent| {
            let close_list = |option| {
                show_list_handle.set(false);
                hovered_value_handle.set(None);
                input_value_on_list_close_handle.set(Some(option));
            };

            if let Some(related_target) = e.related_target() {
                if let Ok(related_target) = related_target.dyn_into::<Element>() {
                    if let Some((option, _)) = options
                        .iter()
                        .find(|(_, id)| id == &related_target.id())
                        .cloned()
                    {
                        input_value_handle.set(option.to_string());
                        on_select.emit(Some(option.clone()));
                        close_list(option.to_string());
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
                }
            }

            is_focused_handle.set(false);
            if let Some((option, _)) = options
                .iter()
                .find(|(option, _)| option.to_string() == input_value)
                .cloned()
            {
                close_list(input_value.clone());
                on_select.emit(Some(option));
            } else {
                close_list("".to_string());
                input_value_handle.set("".to_string());
                on_select.emit(None);
            }
            onblur.emit(());
        })
    };

    let handle_arrow_click = {
        let input_ref = input_ref.clone();

        Callback::from(move |_| {
            if disabled {
                return;
            }
            if let Some(element) = input_ref.cast::<HtmlElement>() {
                if element.focus().is_err() {
                    notify_err(web_err_logic("Failed to focus Select Input"));
                }
                show_list_handle.set(true);
            }
        })
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
        "grow"
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
        (!show_list).then_some("hidden")
    );
    let tip_class = classes!(
        "font-sans",
        "font-normal",
        "text-sm",
        "text-washed-out-secondary"
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
        .map(|(i, (option, id))| {
            let li_class = classes!(
                "font-sans",
                "text-base",
                "cursor-pointer",
                "border-solid",
                "border-0",
                "border-b",
                if Some(&option) == hovered_value.as_ref() {
                    vec!["border-primary", "text-primary"]
                } else {
                    vec!["border-washed-out-thirdly", "text-secondary"]
                }
            );

            let onmouseenter = {
                let hovered_value_handle = hovered_value_handle.clone();
                let option = option.clone();
                Callback::from(move |_| {
                    hovered_value_handle.set(Some(option.clone()));
                })
            };

            html_nested! {
                <li
                  key={id.clone()}
                  id={id.clone()}
                  tabindex={(i+1).to_string()}
                  class={li_class.clone()}
                  {onmouseenter}
                >
                    {option_html.emit(option.to_string())}
                </li>
            }
        })
        .collect();

    let name = label.to_lowercase().replace(' ', "-");
    let mut id = format!("{name}-input");
    if let Some(label_key) = &label_key {
        id = format!("{label_key}-{id}");
    }
    let list_id = format!("{label}-options-list");
    let label = if required { format!("{label}*") } else { label };
    let err_id = error.as_ref().map(|_| format!("{id}-error"));

    html! {
        <div class={classes!("flex","flex-col","w-full",container_class)}>
            <label class={label_class} for={id.clone()}>
                {label}
                if let Some(label_key) = &label_key {
                    <span class="sr-only">{" "}{label_key}</span>
                }
            </label>
            <div class={input_wrapper_class}>
                <input
                    {id}
                    {name}
                    class={input_class}
                    onfocus={handle_focus}
                    onblur={handle_blur}
                    ref={input_ref}
                    value={input_value}
                    oninput={handle_input}
                    onclick={handle_input_click}
                    {onkeydown}
                    aria-owns={list_id.clone()}
                    aria-errormessage={err_id.clone()}
                    {disabled}
                    autocomplete="off"
                    {placeholder}
                />
                <IconButtonForInputDecoration onclick={handle_arrow_click} {disabled} flash_orange_on_click={false}>
                    if show_list {
                        <ChevronUpIcon />
                    } else {
                        <ChevronDownIcon />
                    }
                </IconButtonForInputDecoration>
                {decoration_right}
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
                <small class={tip_class}>{tip}</small>
            }
        </div>
    }
}
