use gloo::file::callbacks;
use gloo::file::FileReadError;
use yew::Callback;
use yew::NodeRef;

use crate::utils::js;
use crate::utils::web_error::WebResult;

pub fn upload_file(
    input_ref: &NodeRef,
    upload_cb: Callback<Result<String, FileReadError>>,
) -> WebResult<callbacks::FileReader> {
    Ok(js::upload_file(input_ref, upload_cb)?)
}
