use yew::prelude::*;

use crate::components::buttons::outlined_secondary_button::OutlinedSecondaryButton;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::components::tables::action_cell::ActionCell;
use crate::components::tables::column_header::ColumnHeader;
use crate::components::tables::table::Table;
use crate::components::tables::table_row::TableRow;
use crate::components::tables::text_cell::TextCell;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn TableStory(props: &Props) -> Html {
    let Props {} = props.clone();
    html! {
        <Stories>
            <Story name={"Plain"} background={StoryBackground::Light}>
                <Table headers={html!{
                    <>
                        <ColumnHeader>{"#"}</ColumnHeader>
                        <ColumnHeader>{"First"}</ColumnHeader>
                        <ColumnHeader>{"Last"}</ColumnHeader>
                        <ColumnHeader>{"Action"}</ColumnHeader>
                    </>
                }}>
                    <TableRow>
                        <TextCell>
                            {"1"}
                        </TextCell>
                        <TextCell>
                            {"Mark"}
                        </TextCell>
                        <TextCell>
                            {"Otto"}
                        </TextCell>
                        <ActionCell>
                            <OutlinedSecondaryButton onclick={Callback::noop()}>{"@mdo"}</OutlinedSecondaryButton>
                        </ActionCell>
                    </TableRow>
                    <TableRow>
                        <TextCell>
                            {"2"}
                        </TextCell>
                        <TextCell>
                            {"Jacob"}
                        </TextCell>
                        <TextCell>
                            {"Thornton"}
                        </TextCell>
                        <ActionCell>
                            <OutlinedSecondaryButton onclick={Callback::noop()}>{"@fat"}</OutlinedSecondaryButton>
                        </ActionCell>
                    </TableRow>
                    <TableRow>
                        <TextCell>
                            {"3"}
                        </TextCell>
                        <TextCell>
                            {"Larry"}
                        </TextCell>
                        <TextCell>
                            {"the Bird"}
                        </TextCell>
                        <ActionCell>
                            <OutlinedSecondaryButton onclick={Callback::noop()}>{"@twitter"}</OutlinedSecondaryButton>
                        </ActionCell>
                    </TableRow>
                </Table>
            </Story>
            <Story name={"Row Clickable"} background={StoryBackground::Light}>
                <Table headers={html!{
                    <>
                        <ColumnHeader>{"#"}</ColumnHeader>
                        <ColumnHeader>{"First"}</ColumnHeader>
                        <ColumnHeader>{"Last"}</ColumnHeader>
                        <ColumnHeader>{"Action"}</ColumnHeader>
                    </>
                }}>
                    <TableRow onclick={Callback::noop()}>
                        <TextCell>
                            {"1"}
                        </TextCell>
                        <TextCell>
                            {"Mark"}
                        </TextCell>
                        <TextCell>
                            {"Otto"}
                        </TextCell>
                        <ActionCell>
                            <OutlinedSecondaryButton onclick={Callback::noop()}>{"@mdo"}</OutlinedSecondaryButton>
                        </ActionCell>
                    </TableRow>
                    <TableRow onclick={Callback::noop()}>
                        <TextCell>
                            {"2"}
                        </TextCell>
                        <TextCell>
                            {"Jacob"}
                        </TextCell>
                        <TextCell>
                            {"Thornton"}
                        </TextCell>
                        <ActionCell>
                            <OutlinedSecondaryButton onclick={Callback::noop()}>{"@fat"}</OutlinedSecondaryButton>
                        </ActionCell>
                    </TableRow>
                    <TableRow onclick={Callback::noop()}>
                        <TextCell>
                            {"3"}
                        </TextCell>
                        <TextCell>
                            {"Larry"}
                        </TextCell>
                        <TextCell>
                            {"the Bird"}
                        </TextCell>
                        <ActionCell>
                            <OutlinedSecondaryButton onclick={Callback::noop()}>{"@twitter"}</OutlinedSecondaryButton>
                        </ActionCell>
                    </TableRow>
                </Table>
            </Story>
        </Stories>
    }
}
