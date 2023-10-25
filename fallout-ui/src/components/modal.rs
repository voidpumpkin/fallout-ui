use gloo::events::EventListener;
use gloo::utils::body;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use yew::prelude::*;
use yew_heroicons::size_24::solid::XMarkIcon;

use crate::components::buttons::outlined_secondary_button::OutlinedSecondaryButton;
use crate::components::buttons::primary_button::PrimaryButton;
use crate::components::typography::header::Header;
use crate::hooks::use_fallout_context;
use crate::utils::modal_tracking_context::ModalTrackingContext;
use crate::utils::toasts::notify_err;
use crate::utils::web_error::web_err_logic;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub disabled: bool,
    // Text props
    pub title: String,
    #[prop_or("Confirm".to_string())]
    pub confirm_cta: String,
    #[prop_or_default]
    pub close_cta: Option<String>,
    // Control props
    pub show: bool,
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub on_confirm: Option<Callback<()>>,
    #[prop_or_default]
    pub form: String,
    // Other
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub data_qa: String,
}

#[function_component]
pub fn Modal(props: &Props) -> Html {
    let Props {
        title,
        confirm_cta,
        show,
        on_close,
        on_confirm,
        form,
        children,
        data_qa,
        loading,
        disabled,
        close_cta,
    } = props.clone();

    let modal_context_id = use_mut_ref(|| 0);

    let ModalTrackingContext {
        active_modal_id,
        start_tracking_modal,
        stop_tracking_modal,
    } = use_fallout_context::<ModalTrackingContext>();

    use_effect_with(show, {
        let modal_context_id = modal_context_id.clone();
        move |_| {
            if show {
                *modal_context_id.borrow_mut() = start_tracking_modal.emit(());
            } else {
                stop_tracking_modal.emit(*modal_context_id.borrow_mut());
            }

            let modal_context_id = modal_context_id.clone();
            move || stop_tracking_modal.emit(*modal_context_id.borrow_mut())
        }
    });

    let show = active_modal_id == *modal_context_id.borrow_mut() && show;

    use_memo((on_close.clone(), show), move |(on_close, show)| {
        let on_close = on_close.clone();

        if !*show {
            return None;
        }

        let listener = EventListener::new(&body(), "keydown", move |event| {
            let keyboard_event: &KeyboardEvent = match event.dyn_ref() {
                Some(some) => some,
                None => return notify_err(web_err_logic("event to KeyboardEvent cast failed")),
            };
            let key = keyboard_event.key();
            if key == "Escape" {
                on_close.emit(());
            }
        });

        Some(listener)
    });

    let modal_root = document()
        .get_element_by_id("modal-root")
        .expect("Expected to find a #modal-root element");

    let close_cta = close_cta.unwrap_or_else(|| {
        if on_confirm.is_some() {
            "Cancel".to_string()
        } else {
            "Close".to_string()
        }
    });

    let container_class = classes!(
        "w-screen",
        "h-screen",
        "fixed",
        "top-0",
        "left-0",
        "flex",
        "justify-center",
        "p-24",
        "box-border",
        "overflow-auto",
        "before:z-modal-bg",
        "before:w-screen",
        "before:h-screen",
        "before:fixed",
        "before:top-0",
        "before:left-0",
        "before:bg-secondary",
        "before:opacity-60",
        "before:content-['']",
        (!show).then_some("hidden")
    );

    create_portal(
        html! {
            <div class={container_class} >
                <div class="z-modal bg-white relative h-fit min-w-[50vw]" data-qa={format!("{data_qa}-modal")}>
                    <button
                        onclick={on_close.reform(|_| {})}
                        class="w-10 h-10 p-0 border-1 border-solid border-washed-out-thirdly hover:border-thirdly rounded-full absolute right-3 top-3 bg-transparent cursor-pointer"
                        data_qa={format!("{data_qa}-modal-close")}
                    >
                        <XMarkIcon class="w-6 text-secondary" />
                    </button>
                    <Header class="!m-9 pr-5">{title}</Header>
                    <div class="px-9 box-border">
                        {children}
                    </div>
                    <div class="mx-9 mt-9 flex gap-3">
                        if let Some(on_confirm) = on_confirm {
                            <PrimaryButton
                                onclick={on_confirm.reform(|_| {})}
                                data_qa={format!("{data_qa}-modal-confirm")}
                                {form}
                                {disabled}
                                {loading}
                            >
                                {confirm_cta}
                            </PrimaryButton>
                        }
                        <OutlinedSecondaryButton
                            onclick={on_close.reform(|_| {})}
                            data_qa={format!("{data_qa}-modal-cancel")}
                        >
                            {close_cta}
                        </OutlinedSecondaryButton>
                    </div>
                    <div class="relative w-full h-10">
                        <svg class="absolute right-0 bottom-0 w-auto h-10" viewBox="0 0 243 44" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path class="fill-primary" d="M0 44L243 0V42C243 43.1046 242.105 44 241 44H0Z"/>
                        </svg>
                    </div>
                </div>
            </div>
        },
        modal_root,
    )
}
