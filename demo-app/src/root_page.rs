use fallout_ui::components::link::primary_link::PrimaryLink;
use fallout_ui::components::typography::header::Header;
use fallout_ui::components::typography::page_header::PageHeader;
use strum::IntoEnumIterator;
use yew::prelude::*;

use crate::root_route::Route;
use crate::root_route::StoryGroup;

#[function_component]
pub fn RootPage() -> Html {
    let stories_iter = Route::iter()
        // skip root
        .skip(1);

    let list = StoryGroup::iter()
        .map(|group| {
            let story_links = stories_iter
                .clone()
                .filter(|stories_route| stories_route.group() == group)
                .map(|stories_route| {
                    html! {
                        <li>
                            <PrimaryLink<Route> to={stories_route.clone()}>
                                {stories_route.to_string()}
                            </PrimaryLink<Route>>
                        </li>
                    }
                })
                .collect::<Html>();

            html! {
                <>
                    <Header class="mt-2">{group}</Header>
                    <ul class="my-2">{story_links}</ul>
                </>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <PageHeader>{"Demo app for fallout-ui"}</PageHeader>
            {list}
        </>
    }
}
