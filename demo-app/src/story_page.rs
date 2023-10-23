use fallout_ui::components::typography::page_header::PageHeader;
use yew::prelude::*;
use yew_router::prelude::use_route;

use crate::root_route::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn StoryPage(props: &Props) -> Html {
    let Props { children } = props.clone();
    let route_name = use_route::<Route>()
        .map(|r| r.to_string())
        .unwrap_or("N/A".to_string());

    html! {
        <>
            <PageHeader>{route_name}</PageHeader>
            {children}
        </>
    }
}
