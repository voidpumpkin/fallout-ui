use fallout_ui::utils::modal_tracking_context::ModalTrackingContextProvider;
use yew::prelude::*;
use yew_router::BrowserRouter as BrowserRouterProvider;

use crate::root_router_switch::RouterSwitch;

#[function_component]
pub fn App() -> Html {
    html! {
        <Suspense fallback={html!{<h1>{"Suspended"}</h1>}}>
        <BrowserRouterProvider>
        <ModalTrackingContextProvider>
        <main class="w-full lg:container p-3 lg:p-10 box-border">
            <RouterSwitch/>
        </main>
        </ModalTrackingContextProvider>
        </BrowserRouterProvider>
        </Suspense>
    }
}
