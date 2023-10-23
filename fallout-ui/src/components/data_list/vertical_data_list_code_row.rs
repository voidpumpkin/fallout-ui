use yew::prelude::*;

use crate::components::typography::body_text::BodyText;
use crate::components::typography::code::Code;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub code: String,
}

#[function_component]
pub fn VerticalDataListCodeRow(props: &Props) -> Html {
    let Props { name, code } = props.clone();

    html! {
        <tr class="contents">
            <BodyText tag="th" class="font-bold whitespace-nowrap break-all">{format!("{name}:")}</BodyText>
            <BodyText tag="td" class="break-all"><Code wrap={true}>{code}</Code></BodyText>
        </tr>
    }
}
