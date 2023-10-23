use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::Response;

use crate::utils::js::window;
use crate::utils::js::JsResult;

pub async fn make_fetch_request(uri: &str, options: &RequestInit) -> JsResult<Response> {
    let request = Request::new_with_str_and_init(uri, options)?;
    let fetch_promise = window()?.fetch_with_request(&request);
    let js_response = JsFuture::from(fetch_promise).await?;
    Ok(js_response.dyn_into()?)
}
