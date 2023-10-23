mod js_error;
mod js_fetch;
mod js_file_upload;
mod js_prompt_file_download;
mod js_utils;

pub(in crate::utils) use js_error::*;
/// JsResult is meant to be used only in this js module, but sometimes there is no choice
/// If possible, better use pub(in crate::utils) and create a wrapper that returns WebResult
pub use js_fetch::make_fetch_request;
pub(in crate::utils) use js_file_upload::*;
pub(in crate::utils) use js_prompt_file_download::*;
pub use js_utils::set_location_href;
pub(in crate::utils) use js_utils::*;
