pub fn fn_once_noop() -> Box<dyn FnOnce()> {
    Box::new(|| {})
}
