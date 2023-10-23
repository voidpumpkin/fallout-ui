use yew::prelude::*;

use crate::components::buttons::danger_button::DangerButton;
use crate::components::buttons::outlined_secondary_button::OutlinedSecondaryButton;
use crate::components::buttons::primary_button::PrimaryButton;
use crate::components::buttons::primary_link_button::PrimaryLinkButton;
use crate::components::buttons::secondary_button::SecondaryButton;
use crate::components::buttons::secondary_link_button::SecondaryLinkButton;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn ButtonStory(props: &Props) -> Html {
    let Props {} = props.clone();
    html! {
        <Stories>
            <Story name={"PrimaryButton"} background={StoryBackground::Light}>
                <PrimaryButton onclick={|_| log::info!("PrimaryButton::onclick fired")}>{"Click me"}</PrimaryButton>
            </Story>
            <Story name={"PrimaryButton Loading"} background={StoryBackground::Light}>
                <PrimaryButton onclick={|_| log::info!("PrimaryButton::onclick fired")} loading={true}>{"Click me"}</PrimaryButton>
            </Story>
            <Story name={"PrimaryButton Disabled"} background={StoryBackground::Light}>
                <PrimaryButton onclick={|_| log::info!("PrimaryButton::onclick fired")} disabled={true}>{"Click me"}</PrimaryButton>
            </Story>
            <Story name={"SecondaryButton"} background={StoryBackground::Light}>
                <SecondaryButton onclick={|_| log::info!("SecondaryButton::onclick fired")}>{"Click me"}</SecondaryButton>
            </Story>
            <Story name={"SecondaryButton Loading"} background={StoryBackground::Light}>
                <SecondaryButton onclick={|_| log::info!("SecondaryButton::onclick fired")} loading={true}>{"Click me"}</SecondaryButton>
            </Story>
            <Story name={"SecondaryButton Disabled"} background={StoryBackground::Light}>
                <SecondaryButton onclick={|_| log::info!("SecondaryButton::onclick fired")} disabled={true}>{"Click me"}</SecondaryButton>
            </Story>
            <Story name={"OutlinedSecondaryButton"} background={StoryBackground::White}>
                <OutlinedSecondaryButton onclick={|_| log::info!("OutlinedSecondaryButton::onclick fired")}>{"Click me"}</OutlinedSecondaryButton>
            </Story>
            <Story name={"OutlinedSecondaryButton Loading"} background={StoryBackground::White}>
                <OutlinedSecondaryButton onclick={|_| log::info!("OutlinedSecondaryButton::onclick fired")} loading={true}>{"Click me"}</OutlinedSecondaryButton>
            </Story>
            <Story name={"OutlinedSecondaryButton Disabled"} background={StoryBackground::White}>
                <OutlinedSecondaryButton onclick={|_| log::info!("OutlinedSecondaryButton::onclick fired")} disabled={true}>{"Click me"}</OutlinedSecondaryButton>
            </Story>
            <Story name={"DangerButton"} background={StoryBackground::White}>
                <DangerButton onclick={|_| log::info!("DangerButton::onclick fired")}>{"Click me"}</DangerButton>
            </Story>
            <Story name={"DangerButton Loading"} background={StoryBackground::White}>
                <DangerButton onclick={|_| log::info!("DangerButton::onclick fired")} loading={true}>{"Click me"}</DangerButton>
            </Story>
            <Story name={"DangerButton Disabled"} background={StoryBackground::White}>
                <DangerButton onclick={|_| log::info!("DangerButton::onclick fired")} disabled={true}>{"Click me"}</DangerButton>
            </Story>
            <Story name={"PrimaryLinkButton"} background={StoryBackground::White}>
                <PrimaryLinkButton onclick={|_| log::info!("PrimaryLinkButton::onclick fired")}>{"Click me"}</PrimaryLinkButton>
            </Story>
            <Story name={"SecondaryLinkButton"} background={StoryBackground::White}>
                <SecondaryLinkButton onclick={|_| log::info!("SecondaryLinkButton::onclick fired")}>{"Click me"}</SecondaryLinkButton>
            </Story>
        </Stories>
    }
}
