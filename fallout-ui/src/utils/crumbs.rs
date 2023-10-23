#[macro_export]
macro_rules! crumb {
    ($to:expr, None) => {
        None
    };
    ($to:expr, $text:expr) => {
        Some(html! {<common_ui::components::link::breadcrumb::Breadcrumb<Self> to={$to.clone()}>{$text}</common_ui::components::link::breadcrumb::Breadcrumb<Self>>})
    };
}

#[macro_export]
macro_rules! crumbs {
    ($to:expr, $($item:tt),* ) => {
        vec![
            $(
                common_ui::crumb!($to, $item)
            ),*
        ]
    };
}
