use gloo::utils::format::JsValueSerdeExt;
use web_sys::Window;

use super::web_error::web_err_js;
use crate::utils::js;
use crate::utils::web_error::web_err_logic;
use crate::utils::web_error::WebResult;

pub fn window() -> WebResult<Window> {
    Ok(js::window()?)
}

pub fn get_window_inner_width() -> WebResult<i32> {
    let width: i32 = js::get_window_inner_width()?
        .into_serde()
        .map_err(web_err_logic)?;
    Ok(width)
}

pub fn get_page_offset_xy() -> WebResult<(i32, i32)> {
    let window = window()?;
    Ok((
        window.page_x_offset().map_err(web_err_js)? as i32,
        window.page_y_offset().map_err(web_err_js)? as i32,
    ))
}
