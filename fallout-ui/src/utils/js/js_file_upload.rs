use std::ops::Deref;

use gloo::file::callbacks;
use gloo::file::FileReadError;
use web_sys::HtmlInputElement;
use yew::Callback;
use yew::NodeRef;

use crate::utils::js::JsError;
use crate::utils::js::JsResult;

pub fn upload_file(
    input_ref: &NodeRef,
    upload_cb: Callback<Result<String, FileReadError>>,
) -> JsResult<callbacks::FileReader> {
    match input_ref.cast::<HtmlInputElement>() {
        Some(input_elem) => match input_elem.files() {
            Some(file_list) => {
                let file_list = gloo::file::FileList::from(file_list);
                match file_list.deref().first() {
                    Some(file) => Ok(callbacks::read_as_text(file.deref(), move |resp| {
                        upload_cb.emit(resp)
                    })),
                    None => Err(JsError::new("Failed to parse file info")),
                }
            }
            None => Err(JsError::new("No File Selected")),
        },
        None => Err(JsError::new("Failed to cast to file input element")),
    }
}
