use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ColumnHeader(props: &Props) -> Html {
    let Props { children } = props.clone();
    html! {
        <th scope="col" class="text-sm font-medium text-secondary px-6 py-4 text-left whitespace-nowrap">
            {children}
        </th>
    }
}
