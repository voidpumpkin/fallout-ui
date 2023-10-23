use yew::prelude::*;
use yew_heroicons::size_20::solid::ChevronDownIcon;
use yew_heroicons::size_20::solid::ChevronRightIcon;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub expanded: bool,
    #[prop_or_default]
    pub expandable_content: Html,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub data_qa: String,
}

#[function_component]
pub fn AccordionTableRow(props: &Props) -> Html {
    let Props {
        expanded,
        expandable_content,
        children,
        onclick,
        data_qa,
    } = props.clone();

    let class = classes!(
        "bg-washed-out-thirdly",
        "[&:nth-child(4n+1)]:bg-white",
        "border-0",
        "border-b",
        "border-b-secondary",
        "border-solid",
        "transition",
        "duration-300",
        "ease-in-out",
        onclick.is_some().then(|| classes!(
            "hover:bg-thirdly",
            "hover:cursor-pointer",
            "[&:nth-child(4n+1)]:hover:bg-thirdly"
        ))
    );

    let data_qa = format!("{data_qa}-item");

    let expanded_row_class = if expanded { "" } else { "hidden" };

    let expanded_content_container_class = classes!(
        "bg-white",
        "p-4",
        "w-full",
        "shadow-[inset_0px_4px_4px_rgba(0,0,0,0.15),inset_0px_-4px_4px_rgba(0,0,0,0.15)]",
        "border-0",
        "border-b-1",
        "border-solid",
        "border-b-secondary",
        "overflow-hidden",
    );

    html! {
        <>
            <tr {class} {onclick} data-qa={data_qa}>
                <td class="text-secondary px-6 py-4">
                    if expanded {
                        <ChevronDownIcon class={"w-6 min-w-[1.5rem]"}/>
                    } else {
                        <ChevronRightIcon class={"w-6 min-w-[1.5rem]"}/>
                    }
                </td>
                {children}
            </tr>
            <tr class={expanded_row_class}>
                <td colspan="99" class={expanded_content_container_class}>
                    {expandable_content}
                </td>
            </tr>
        </>
    }
}
