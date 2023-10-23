use yew::prelude::*;
use yew_router::prelude::*;

/// Wrapper for `yew_router::use_location` with predefined error message and unwrap
#[hook]
pub fn use_fallout_location() -> Location {
    use_location().expect("Location is None. This error can happen if router was not provided.")
}
