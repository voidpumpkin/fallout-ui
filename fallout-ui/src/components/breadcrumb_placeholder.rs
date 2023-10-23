use yew::prelude::*;

#[function_component]
pub fn BreadcrumbPlaceholder() -> Html {
    html! {
        <span class={"font-sans m-0 rounded w-9 h-2 bg-washed-out-secondary animate-pulse cursor-progress"} aria-label="loading breadcrumb" />
    }
}
