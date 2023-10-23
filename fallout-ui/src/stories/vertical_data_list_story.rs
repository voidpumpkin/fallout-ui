use chrono::Utc;
use yew::prelude::*;

use crate::components::data_list::vertical_data_list::VerticalDataList;
use crate::components::data_list::vertical_data_list_row::VerticalDataListRow;
use crate::components::data_list::vertical_data_list_row_date::VerticalDataListRowDate;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[function_component]
pub fn VerticalDataListStory() -> Html {
    let now = Utc::now().naive_utc();

    html! {
        <Stories>
            <Story background={StoryBackground::White}>
                <VerticalDataList>
                    <VerticalDataListRow name="Text" value="Hello world"/>
                    <VerticalDataListRow name="Long text" value="Lorem Ipsum is simply dummy text of the printing and typesetting industry. \
                    Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. \
                    It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. \
                    It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum."/>
                    <VerticalDataListRowDate name="Date time" value={now}/>
                </VerticalDataList>
            </Story>
        </Stories>
    }
}
