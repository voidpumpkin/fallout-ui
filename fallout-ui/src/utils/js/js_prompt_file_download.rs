use wasm_bindgen::JsCast;
use web_sys::Blob;
use web_sys::HtmlAnchorElement;
use web_sys::Url;

use crate::utils::js::document;
use crate::utils::js::JsError;
use crate::utils::js::JsResult;

/// Mime type can be found inside Blob
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Blob/type)
#[derive(Debug)]
pub struct DownloadedFile {
    pub filename: Option<String>,
    pub blob: Blob,
}

pub fn prompt_file_download(file: &DownloadedFile) -> JsResult<()> {
    let filename = file.filename.clone().unwrap_or_default();

    let url = Url::create_object_url_with_blob(&file.blob)?;

    let element = document()?.create_element("a")?;
    let anchor_element = element.dyn_into::<HtmlAnchorElement>()?;
    anchor_element.set_href(&url);
    anchor_element.set_download(&filename);

    document()?
        .body()
        .ok_or_else(|| JsError::new("body does not exist on document"))?
        .append_child(&anchor_element)?;
    anchor_element.click();
    anchor_element.remove();
    Ok(())
}
