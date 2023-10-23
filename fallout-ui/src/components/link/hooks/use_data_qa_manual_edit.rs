use web_sys::Element;
use yew::prelude::*;

use crate::utils::toasts::notify_err;
use crate::utils::web_error::web_err_js;
use clone_on_capture::clone_on_capture;

#[clone_on_capture]
#[hook]
pub fn use_data_qa_manual_edit(data_qa: String) -> NodeRef {
    let anchor_ref = use_node_ref();

    use_effect(move || {
        let Some(anchor_ref) = anchor_ref.cast::<Element>() else {
            return;
        };
        anchor_ref
            .set_attribute("data-qa", data_qa.as_str())
            .err()
            .map(web_err_js)
            .map(notify_err)
            .unwrap_or_default()
    });

    anchor_ref
}
