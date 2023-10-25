use yew::prelude::*;

#[hook]
pub fn use_memo_owned<T, F, D>(f: F, deps: D) -> T
where
    T: 'static + Clone,
    F: FnOnce(&D) -> T,
    D: 'static + PartialEq,
{
    use_memo(deps, f).as_ref().clone()
}
