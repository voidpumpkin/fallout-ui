use yew::prelude::*;

use crate::components::typography::body_text::BodyText;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub value: String,
    #[prop_or_default]
    pub data_qa: Option<String>,
}

#[function_component]
pub fn VerticalDataListRow(props: &Props) -> Html {
    let Props {
        name,
        value,
        data_qa,
    } = props.clone();

    html! {
        <tr class="contents">
            <BodyText tag="th" class="font-bold whitespace-nowrap break-all">{format!("{name}:")}</BodyText>
            <BodyText tag="td" class="break-all" {data_qa}>{value}</BodyText>
        </tr>
    }
}
