use gloo::timers;
use yew::prelude::*;

#[hook]
pub fn use_interval<F, D>(func: F, millis: u32, deps: D)
where
    F: 'static + FnMut(),
    D: 'static + PartialEq,
{
    use_memo(deps, move |_| timers::callback::Interval::new(millis, func));
}
