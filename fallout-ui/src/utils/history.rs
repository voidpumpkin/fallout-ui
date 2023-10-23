use crate::utils::js;
use crate::utils::web_error::WebResult;

pub fn back() -> WebResult<()> {
    Ok(js::history_go_back()?)
}
