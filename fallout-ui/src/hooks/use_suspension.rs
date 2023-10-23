use yew::prelude::*;
use yew::suspense::Suspension;
use yew::use_mut_ref;
use yew::Callback;

#[hook]
pub fn use_suspension() -> Callback<(), Suspension> {
    let suspension_handle_ref = use_mut_ref(|| None);

    Callback::from(move |_| -> Suspension {
        let (s, handle) = Suspension::new();
        *suspension_handle_ref.borrow_mut() = Some(handle);
        s
    })
}
