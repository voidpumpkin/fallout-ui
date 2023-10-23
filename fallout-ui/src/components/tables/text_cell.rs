use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn TextCell(props: &Props) -> Html {
    let Props { children } = props.clone();
    html! {
        <td class="text-sm text-secondary px-6 py-4 whitespace-nowrap">
            {children}
        </td>
    }
}
