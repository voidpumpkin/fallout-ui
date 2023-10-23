use yew::prelude::*;

#[hook]
pub fn use_modal(default_value: bool) -> (bool, Callback<()>, Callback<()>, Callback<bool>) {
    let is_open_handle = use_state(|| default_value);
    let is_open = *is_open_handle;

    let is_open_setter = is_open_handle.setter();

    let set_is_open = use_callback(
        |new_val, is_open_setter| {
            is_open_setter.set(new_val);
        },
        is_open_setter.clone(),
    );

    let close = use_callback(
        move |_, set_is_open| {
            set_is_open.set(false);
        },
        is_open_setter.clone(),
    );

    let open = use_callback(
        |_, is_open_setter| {
            is_open_setter.set(true);
        },
        is_open_setter,
    );

    (is_open, open, close, set_is_open)
}
