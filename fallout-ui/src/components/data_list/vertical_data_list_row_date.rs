use chrono::NaiveDateTime;
use yew::prelude::*;

use crate::components::data_list::vertical_data_list_row::VerticalDataListRow;
use crate::utils::make_zulu_timestamp;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub name: String,
    pub value: NaiveDateTime,
}

#[function_component]
pub fn VerticalDataListRowDate(props: &Props) -> Html {
    let Props { name, value } = props.clone();

    let value = make_zulu_timestamp(value);

    html! {
        <VerticalDataListRow {name} {value}/>
    }
}
