use yew::prelude::*;

use crate::hooks::use_memo_owned;

#[hook]
pub fn use_secondary_link_logic(
    class: Classes,
    has_error: bool,
    text_classes: Option<Callback<bool, Classes>>,
) -> Classes {
    let class = use_memo_owned(
        |(class, has_error)| {
            let text_classes = match text_classes {
                None => {
                    if *has_error {
                        classes!["text-danger", "hover:text-reacting-danger"]
                    } else {
                        classes!["text-secondary", "hover:text-reacting-secondary"]
                    }
                }
                Some(callback) => callback.emit(*has_error),
            };

            classes!(
                "font-sans",
                "m-0",
                "text-base",
                "hover:underline",
                text_classes,
                class.clone()
            )
        },
        (class, has_error),
    );
    class
}
