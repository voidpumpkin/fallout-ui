use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use web_sys::EventTarget;
use yew::prelude::*;

use crate::utils::get_window_inner_width;
use crate::utils::toasts::notify_err;

fn get_window_width() -> i32 {
    match get_window_inner_width() {
        Ok(new_width) => new_width,
        Err(err) => {
            notify_err(err);
            0
        }
    }
}

/// Listens for window size changes
/// returns true for devices wider than our set max mobile width in pixels
#[hook]
pub fn use_is_desktop() -> bool {
    let window_width_handle = use_state_eq(get_window_width);
    let window_width = *window_width_handle;

    use_effect_with(window_width_handle.clone(), {
        move |_| {
            let resize_event_target: EventTarget =
                window().unwrap_throw().dyn_into().unwrap_throw();
            let listener = EventListener::new(&resize_event_target, "resize", move |_| {
                window_width_handle.set(get_window_width())
            });
            move || drop(listener)
        }
    });

    window_width >= 1024
}
