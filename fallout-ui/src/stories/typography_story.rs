use yew::prelude::*;

use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::components::typography::body_text::BodyText;
use crate::components::typography::header::Header;
use crate::components::typography::page_header::PageHeader;
use crate::components::typography::small_body_text::SmallBodyText;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn TypographyStory(props: &Props) -> Html {
    let Props {} = props.clone();
    html! {
        <Stories>
            <Story name={"PageHeader"} background={StoryBackground::White}>
                <PageHeader>{"Lorem ipsum dolor sit amet"}</PageHeader>
            </Story>
            <Story name={"Header"} background={StoryBackground::White}>
                <Header>{"Lorem ipsum dolor sit amet"}</Header>
            </Story>
            <Story name={"Body"} background={StoryBackground::White}>
                <BodyText>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}</BodyText>
            </Story>
            <Story name={"SmallBody"} background={StoryBackground::White}>
                <SmallBodyText>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}</SmallBodyText>
            </Story>
        </Stories>
    }
}
