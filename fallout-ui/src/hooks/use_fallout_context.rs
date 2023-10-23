use yew::prelude::*;

/// Wrapper for `yew::use_context` with predefined error message and unwrap
#[hook]
pub fn use_fallout_context<R: Clone + PartialEq + 'static>() -> R {
    use_context::<R>().expect(stringify!(
        "Context was None, did you forget to provide it?"
    ))
}
