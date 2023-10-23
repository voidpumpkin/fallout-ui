use std::ops::Deref;

use yew::prelude::*;

#[hook]
pub fn use_buttons_shared_logic(onclick: Callback<MouseEvent>, disabled: bool, loading: bool) -> Callback<MouseEvent> {
    let onclick = use_memo(
        move |(onclick, ..)| {
            let onclick = onclick.clone();

            Callback::from(move |e| {
                if disabled || loading {
                    return;
                }
                onclick.emit(e);
            })
        },
        (onclick, disabled, loading),
    );

    onclick.deref().clone()
}
