use yew::prelude::*;

use crate::components::tables::column_header::ColumnHeader;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub headers: Html,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(classes!("overflow-x-auto"))]
    pub class: Classes,
}

#[function_component]
pub fn AccordionTable(props: &Props) -> Html {
    let Props {
        headers,
        children,
        class,
    } = props.clone();

    html! {
        <div {class}>
            <table class="min-w-full border-collapse font-sans">
                <thead class="bg-white border-0 border-b border-b-secondary border-solid">
                    <tr>
                        <ColumnHeader />
                        {headers}
                    </tr>
                </thead>
                <tbody>
                    {children}
                </tbody>
            </table>
        </div>
    }
}
