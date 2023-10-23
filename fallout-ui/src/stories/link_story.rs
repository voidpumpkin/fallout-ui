use yew::prelude::*;
use yew_router::Routable;

use crate::components::link::primary_link::PrimaryLink;
use crate::components::link::secondary_link::SecondaryLink;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props<R>
where
    R: Routable + 'static,
{
    pub current_page: R,
}

#[function_component]
pub fn LinkStory<R>(props: &Props<R>) -> Html
where
    R: Routable + 'static,
{
    let Props { current_page } = props.clone();
    html! {
        <Stories>
            <Story name={"Variant::BrightOrange *Default"} background={StoryBackground::White}>
                <PrimaryLink<R> to={current_page.clone()}>{"Click me"}</PrimaryLink<R>>
            </Story>
            <Story name={"Variant::DarkBrown"} background={StoryBackground::White}>
                <SecondaryLink<R> to={current_page}>{"Click me"}</SecondaryLink<R>>
            </Story>
        </Stories>
    }
}
