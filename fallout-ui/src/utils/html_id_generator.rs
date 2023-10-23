use std::sync::Mutex;

lazy_static! {
    pub static ref HTML_ID_GENERATOR: HtmlIdGenerator = HtmlIdGenerator::new();
}

/// auto-generates id's for html elements
pub struct HtmlIdGenerator {
    id: Mutex<u64>,
}

impl HtmlIdGenerator {
    fn new() -> HtmlIdGenerator {
        Self { id: Mutex::new(1) }
    }

    pub fn get_id(&self) -> String {
        let mut value = self.id.lock().unwrap();
        *value += 1;
        (*value).to_string()
    }
}
