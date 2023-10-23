#![recursion_limit = "1024"]

#[cfg(not(target_arch = "wasm32"))]
compile_error!(
    "rustc is not correctly configured for this crate, target target_arch has to be wasm32"
);

mod app;
pub mod root_page;
mod root_route;
mod root_router_switch;
pub mod story_page;

use fallout_ui::utils::toasts::init_notifier;
use gloo::utils::document;

use crate::app::App;

pub fn main() {
    init_notifier(30_000);

    let root = document()
        .get_element_by_id("root")
        .expect("root element not found");

    yew::Renderer::<App>::with_root(root).render();
}
