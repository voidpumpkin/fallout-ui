use yew::prelude::*;
use yew::props;

use crate::components::expandable::expandable_list;

#[hook]
pub fn use_expandable_list() -> (expandable_list::Props, UseStateHandle<bool>) {
    let mut list_class = Classes::new();

    let expanded_handle = use_state_eq(bool::default);
    let expanded = *expanded_handle;

    if !expanded {
        list_class.push("hidden")
    };

    let toggle_expanded = {
        let expanded_handle = expanded_handle.clone();

        Callback::from(move |_| {
            expanded_handle.set(!*expanded_handle);
        })
    };

    let props = props! {expandable_list::Props {
        expanded,
        toggle_expanded,
        list_class,
    }};

    (props, expanded_handle)
}
