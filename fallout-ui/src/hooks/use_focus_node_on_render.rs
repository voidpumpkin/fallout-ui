use web_sys::HtmlElement;
use yew::prelude::*;

use crate::utils::toasts::notify_err;
use crate::utils::web_error::web_err_logic;

#[hook]
pub fn use_focus_node_on_render(node: NodeRef) {
    use_effect_with((), move |_| {
        match node.cast::<HtmlElement>() {
            Some(elem) => {
                if elem.focus().is_err() {
                    notify_err(web_err_logic("Failed to focus HtmlElement"));
                }
            }
            None => notify_err(web_err_logic("Failed to cast NodeRef to HtmlElement")),
        };
        || {}
    });
}
