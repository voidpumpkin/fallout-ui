use yew::prelude::*;

use crate::hooks::use_memo_owned;

#[hook]
pub fn use_primary_link_logic(class: Classes) -> Classes {
    let class = use_memo_owned(
        |class| {
            classes!(
                "font-sans",
                "m-0",
                "text-base",
                "hover:underline",
                "text-primary",
                "hover:text-primary",
                class.clone()
            )
        },
        class,
    );

    class
}
