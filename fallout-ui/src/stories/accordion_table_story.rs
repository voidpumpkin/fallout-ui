use clone_on_capture::clone_on_capture;
use yew::prelude::*;

use crate::components::data_list::vertical_data_list::VerticalDataList;
use crate::components::data_list::vertical_data_list_row::VerticalDataListRow;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::components::tables::accordion_table::accordion_table_row::AccordionTableRow;
use crate::components::tables::accordion_table::AccordionTable;
use crate::components::tables::column_header::ColumnHeader;
use crate::components::tables::text_cell::TextCell;
use crate::components::typography::header::Header;
use crate::hooks::use_memo_owned;

#[clone_on_capture]
#[function_component]
pub fn AccordionTableStory() -> Html {
    let testing_data = use_memo_owned(
        |_| {
            vec![
                ("peach@smiths.com", "peach", "smith", "+100000001"),
                ("lime@smiths.com", "lime", "smith", "+100000002"),
                ("celery@smiths.com", "celery", "smith", "+100000003"),
                ("cherry@smiths.com", "cherry", "smith", "+100000004"),
            ]
        },
        (),
    );

    let expanded_row_handle = use_state(|| None);

    let rows: Html = testing_data
        .into_iter()
        .map(move |row_data| {
            let expanded = expanded_row_handle.as_deref() == Some(row_data.0);

            let expandable_content = html! {
                <>
                    <Header>{"Funds Originator"}</Header>
                    <VerticalDataList>
                        <VerticalDataListRow name="Email" value={row_data.0}/>
                        <VerticalDataListRow name="Forename" value={row_data.1}/>
                        <VerticalDataListRow name="Surname" value={row_data.2}/>
                        <VerticalDataListRow name="Phone No" value={row_data.3}/>
                    </VerticalDataList>
                </>
            };

            let onclick = move |_| {
                if expanded {
                    expanded_row_handle.set(None)
                } else {
                    expanded_row_handle.set(Some(row_data.0.to_string()))
                }
            };

            html! {
                <AccordionTableRow key={row_data.0} {onclick} {expanded} {expandable_content}>
                    <TextCell>{row_data.0}</TextCell>
                    <TextCell>{row_data.1}</TextCell>
                    <TextCell>{row_data.2}</TextCell>
                    <TextCell>{row_data.3}</TextCell>
                </AccordionTableRow>
            }
        })
        .collect();

    html! {
        <Stories>
            <Story name={"Plain"} background={StoryBackground::Light}>
                <AccordionTable headers={html!{
                    <>
                        <ColumnHeader>{"Email"}</ColumnHeader>
                        <ColumnHeader>{"Forename"}</ColumnHeader>
                        <ColumnHeader>{"Surname"}</ColumnHeader>
                        <ColumnHeader>{"Phone No"}</ColumnHeader>
                    </>
                }}>
                    {rows}
                </AccordionTable>
            </Story>
        </Stories>
    }
}
