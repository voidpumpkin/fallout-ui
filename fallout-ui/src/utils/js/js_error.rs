use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use chrono::Utc;
use wasm_bindgen::JsValue;
use web_sys::console;

pub type JsResult<T> = Result<T, JsError>;

#[derive(Clone, PartialEq)]
pub struct JsError {
    // user friendly message
    pub js: JsValue,
    id: String,
}

impl JsError {
    pub fn new(msg: &str) -> Self {
        JsError {
            js: msg.into(),
            id: JsError::generate_id(),
        }
    }

    fn generate_id() -> String {
        format!("JsError-{}", Utc::now().timestamp())
    }

    fn log_js_value(&self) {
        console::error_2(&format!("{} jsvalue:", self.id).into(), &self.js)
    }
}

impl Debug for JsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.log_js_value();
        f.debug_struct("JsError")
            .field("js", &self.js)
            .field("id", &self.id)
            .finish()
    }
}

impl Display for JsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.log_js_value();
        write!(f, "JsError (details logged above with id \"{}\")", self.id)
    }
}

impl Error for JsError {}

impl<T> From<T> for JsError
where
    T: Into<JsValue>,
{
    fn from(js: T) -> Self {
        JsError {
            js: js.into(),
            id: JsError::generate_id(),
        }
    }
}
