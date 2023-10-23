use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ActionCell(props: &Props) -> Html {
    let Props { children } = props.clone();

    let cancel_click_bubbling = move |e: MouseEvent| {
        e.stop_propagation();
    };

    html! {
        <td
          class="text-sm text-secondary font-light px-6 py-2 whitespace-nowrap cursor-default"
          onclick={cancel_click_bubbling}
        >
            {children}
        </td>
    }
}
