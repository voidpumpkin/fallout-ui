use yew::prelude::*;

use crate::components::typography::body_text::BodyText;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub background: StoryBackground,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub inner_class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoryBackground {
    Light,
    Dark,
    White,
    Brown,
}

const DEFAULT_CLASSES: [&str; 5] = ["box-border", "p-2", "py-4", "w-full", "shadow-inner"];

impl From<StoryBackground> for Classes {
    fn from(v: StoryBackground) -> Self {
        match v {
            StoryBackground::Light | StoryBackground::White | StoryBackground::Brown => {
                classes!(DEFAULT_CLASSES.as_ref(), "checkered-background-light")
            }
            StoryBackground::Dark => {
                classes!(DEFAULT_CLASSES.as_ref(), "checkered-background-dark")
            }
        }
    }
}

const DEFAULT_INNER_CLASSES: [&str; 2] = ["box-border", "p-3"];

fn get_inner_container_class(v: StoryBackground) -> Classes {
    match v {
        StoryBackground::Light => classes!(DEFAULT_INNER_CLASSES.as_ref(), "bg-transparent"),
        StoryBackground::Dark => classes!(DEFAULT_INNER_CLASSES.as_ref(), "bg-transparent"),
        StoryBackground::White => classes!(DEFAULT_INNER_CLASSES.as_ref(), "bg-white"),
        StoryBackground::Brown => classes!(DEFAULT_INNER_CLASSES.as_ref(), "bg-secondary"),
    }
}

#[function_component]
pub fn Story(props: &Props) -> Html {
    let Props {
        background,
        name,
        class,
        inner_class,
        children,
    } = props.clone();
    html! {
        <div>
            <BodyText class="rounded-t-md px-2 w-fit font-bold shadow-[inset_0_1px_4px_0_rgba(0,0,0,0.25)]">{name}</BodyText>
            <div class={classes!(Classes::from(background),class)}>
                <div class={classes!(get_inner_container_class(background), inner_class)}>
                    {children}
                </div>
            </div>
        </div>
    }
}
