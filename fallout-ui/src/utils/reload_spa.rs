use crate::utils::js;
use crate::utils::web_error::WebResult;

pub fn reload_spa() -> WebResult<()> {
    Ok(js::reload_spa()?)
}
