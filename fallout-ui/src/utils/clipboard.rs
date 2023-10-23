use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(inline_js = r#"
export function copy_to_clipboard(value) {
  var textArea = document.createElement("textarea");
  textArea.value = value;

  // Avoid scrolling to bottom
  textArea.style.top = "0";
  textArea.style.left = "0";
  textArea.style.position = "fixed";

  document.body.appendChild(textArea);
  textArea.focus();
  textArea.select();

  try {
    document.execCommand('copy');
  } catch (err) {
    console.error('unable to copy', err);
  }

  document.body.removeChild(textArea);
}
"#)]

extern "C" {
    #[wasm_bindgen(catch)]
    pub fn copy_to_clipboard(value: String) -> Result<(), js_sys::Error>;
}
