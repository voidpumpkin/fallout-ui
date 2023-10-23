use yew::prelude::*;

#[hook]
pub fn use_is_first_mount() -> bool {
    let is_first_ref = use_mut_ref(|| true);
    let is_first = *is_first_ref.borrow();

    if is_first {
        *is_first_ref.borrow_mut() = false;
    }

    is_first
}
