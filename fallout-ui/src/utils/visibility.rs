use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = SetElementVisibilityById)]
    pub fn set_elem_visibility(id: String, state: String);
}
