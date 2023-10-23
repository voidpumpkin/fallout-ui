use yew::prelude::*;

use crate::components::buttons::primary_button::PrimaryButton;
use crate::components::modal::Modal;
use crate::components::stories::Stories;
use crate::components::story::Story;
use crate::components::story::StoryBackground;
use crate::hooks::use_modal::use_modal;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct Props {}

#[function_component]
pub fn ModalStory(props: &Props) -> Html {
    let Props {} = props.clone();

    let (show, open, close, ..) = use_modal(false);
    let (show1, open1, close1, ..) = use_modal(false);
    let (show2, open2, close2, ..) = use_modal(false);

    html! {
        <Stories>
            <Story name="Simple Modal" background={StoryBackground::White}>
                <PrimaryButton onclick={open.reform(|_|{})}>{"Open"}</PrimaryButton>
                <Modal
                    {show}
                    title="Modal"
                    on_close={close}
                    on_confirm={Callback::from(|_| log::info!("Modal::on_confirm fired"))}
                >
                    <p>{"text"}</p>
                    <p>{"text"}</p>
                    <p>{"text"}</p>
                </Modal>
            </Story>
            <Story name={"Two Modals Simultaneously"} background={StoryBackground::White}>
                <PrimaryButton onclick={open1.reform(move |_|{
                    open2.emit(());
                })}>{"Open Two Modals Simultaneously"}</PrimaryButton>
                <Modal
                    show={show1}
                    title="Modal1"
                    on_close={close1.reform(|_| log::info!("Modal1::on_close fired"))}
                    on_confirm={Callback::from(|_| log::info!("Modal1::on_confirm fired"))}
                />
                <Modal
                    show={show2}
                    title="Modal2"
                    on_close={close2.reform(|_| log::info!("Modal2::on_close fired"))}
                    on_confirm={Callback::from(|_| log::info!("Modal2::on_confirm fired"))}
                />
            </Story>
        </Stories>
    }
}
