use js_sys::global;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Document;
use web_sys::Event;
use web_sys::HtmlElement;
use web_sys::Window;

use crate::utils::js::JsError;
use crate::utils::js::JsResult;

pub fn window() -> JsResult<Window> {
    Ok(global().dyn_into()?)
}
pub fn document() -> JsResult<Document> {
    window()?
        .document()
        .ok_or_else(|| JsError::new("window.document is None"))
}
pub fn history_go_back() -> JsResult<()> {
    Ok(window()?.history()?.back()?)
}
pub fn set_location_href(href: &str) -> JsResult<()> {
    Ok(window()?.location().set_href(href)?)
}
pub fn get_window_inner_width() -> JsResult<JsValue> {
    Ok(window()?.inner_width()?)
}
pub fn reload_spa() -> JsResult<()> {
    Ok(window()?.location().reload()?)
}
pub fn get_event_target_data(event: &JsValue, key: &str) -> JsResult<String> {
    let e: &Event = event
        .dyn_ref()
        .ok_or_else(|| JsError::new("Provided Event is None"))?;
    let target: HtmlElement = e
        .target()
        .ok_or_else(|| JsError::new("Provided Event has no target"))?
        .dyn_into()?;
    let dataset = target
        .dataset()
        .get(key)
        .ok_or_else(|| JsError::new("Target does not have requested dataset attribute"))?;
    Ok(dataset)
}
