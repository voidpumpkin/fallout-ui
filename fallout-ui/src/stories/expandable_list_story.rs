use yew::prelude::*;

use crate::components::expandable::expandable_list::ExpandableList;
use crate::components::expandable::expandable_list_item::ExpandableListItem;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::components::typography::body_text::BodyText;
use crate::hooks::use_expandable_list;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn ExpandableListStory(props: &Props) -> Html {
    let Props {} = props.clone();

    let (expandable_list_props, ..) = use_expandable_list();

    html! {
        <Stories>
            <Story background={StoryBackground::White} class="w-80">
                <ExpandableList ..expandable_list_props>
                    <ExpandableListItem>
                        <BodyText>{"Item 1"}</BodyText>
                    </ExpandableListItem>
                    <ExpandableListItem>
                        <BodyText>{"Item 2"}</BodyText>
                    </ExpandableListItem>
                    <ExpandableListItem>
                        <BodyText>{"Item 3"}</BodyText>
                    </ExpandableListItem>
                </ExpandableList>
            </Story>
        </Stories>
    }
}
