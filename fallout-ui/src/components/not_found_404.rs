use yew::prelude::*;

use crate::components::callouts::callout_warning::CalloutWarning;
use crate::components::typography::page_header::PageHeader;
use crate::hooks::use_fallout_location;

#[function_component]
pub fn NotFound404() -> Html {
    let location = use_fallout_location();
    let path = location.path();
    html! {
        <>
            <PageHeader>{"404"}</PageHeader>
            <CalloutWarning>
                {format!("Page \"{path}\" not found")}
            </CalloutWarning>
        </>
    }
}
