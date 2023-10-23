use gloo::utils::format::JsValueSerdeExt;
use js_sys::Reflect;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Event;
use web_sys::File;
use web_sys::HtmlInputElement;
use web_sys::HtmlTextAreaElement;
use web_sys::InputEvent;

use crate::utils::js;
use crate::utils::web_error::web_err_js;
use crate::utils::web_error::web_err_logic;
use crate::utils::web_error::WebResult;

pub fn get_input_value(input_event: InputEvent) -> WebResult<String> {
    let event: Event = input_event.dyn_into().map_err(web_err_js)?;
    get_input_value_from_event(event)
}

pub fn get_input_value_from_event(event: Event) -> WebResult<String> {
    let target = event.target().ok_or_else(|| web_err_js(event))?;
    let input_element: HtmlInputElement = target.dyn_into().map_err(web_err_js)?;
    let value = input_element.value();
    Ok(value)
}

pub fn get_checked_value_from_event(event: Event) -> WebResult<bool> {
    let target = event.target().ok_or_else(|| web_err_js(event))?;
    let input_element: HtmlInputElement = target.dyn_into().map_err(web_err_js)?;
    let value = input_element.checked();
    Ok(value)
}

pub fn get_selection_start(input_event: InputEvent) -> WebResult<Option<u32>> {
    let event: Event = input_event.dyn_into().map_err(web_err_js)?;
    let target = event.target().ok_or_else(|| web_err_js(event))?;
    let input_element: HtmlInputElement = target.dyn_into().map_err(web_err_js)?;
    let selection_start = input_element.selection_start().map_err(web_err_js)?;
    Ok(selection_start)
}

pub fn get_text_area_value(input_event: InputEvent) -> WebResult<String> {
    let event: Event = input_event.dyn_into().map_err(web_err_js)?;
    get_text_area_value_from_event(event)
}

pub fn get_text_area_value_from_event(event: Event) -> WebResult<String> {
    let target = event.target().ok_or_else(|| web_err_js(event))?;
    let input_element: HtmlTextAreaElement = target.dyn_into().map_err(web_err_js)?;
    let value = input_element.value();
    Ok(value)
}

#[macro_export]
macro_rules! handle_oninput {
    ($handle:ident) => {{
        let $handle = $handle.clone();
        yew::Callback::from(move |(v, _): (String, yew::InputEvent)| {
            $handle.set(v);
        })
    }};
}

pub fn get_event_file(event: Event) -> WebResult<File> {
    let target = event.target().ok_or_else(|| web_err_js(event))?;
    let files = Reflect::get(&target, &"files".into()).map_err(web_err_js)?;
    let file_js = Reflect::get(&files, &"0".into()).map_err(web_err_js)?;
    let file: File = file_js.dyn_into().map_err(web_err_js)?;
    Ok(file)
}

pub fn get_input_file_from_event(event: Event) -> WebResult<gloo::file::File> {
    Ok(get_event_file(event)?.into())
}

pub fn get_event_selected_index<T>(event: Event) -> WebResult<T>
where
    T: DeserializeOwned + 'static,
{
    let target = event.target().ok_or_else(|| web_err_js(event))?;
    let selection_js = Reflect::get(&target, &"selectedIndex".into()).map_err(web_err_js)?;
    let selection: T = selection_js.into_serde().map_err(web_err_logic)?;
    Ok(selection)
}
pub fn get_event_target_data(event: &JsValue, key: &str) -> WebResult<String> {
    Ok(js::get_event_target_data(event, key)?)
}
