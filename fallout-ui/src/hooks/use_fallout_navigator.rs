use yew::prelude::*;
use yew_router::prelude::*;

/// Wrapper for `yew_router::use_navigator` with predefined error message and unwrap
#[hook]
pub fn use_fallout_navigator() -> Navigator {
    use_navigator().expect("Navigator is None. This error can happen if router was not provided.")
}
