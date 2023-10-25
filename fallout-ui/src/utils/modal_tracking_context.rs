use std::rc::Rc;

use lazy_static::__Deref;
use yew::prelude::*;

#[derive(Debug)]
enum Action {
    StartTracking(usize),
    StopTracking(usize),
}

#[derive(Default, Clone, Debug, PartialEq)]
struct TrackedModalIds(Vec<usize>);

impl TrackedModalIds {
    fn last(&self) -> Option<usize> {
        self.0.last().cloned()
    }
}

impl Reducible for TrackedModalIds {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut modal_ids = self.deref().clone().0;

        match action {
            Action::StartTracking(id) => {
                if !modal_ids.iter().any(|modal_id| modal_id == &id) {
                    modal_ids.push(id);
                }
            }
            Action::StopTracking(id) => {
                modal_ids.retain(|modal_id| modal_id != &id);
            }
        };

        TrackedModalIds(modal_ids).into()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModalTrackingContext {
    pub active_modal_id: usize,
    pub start_tracking_modal: Callback<(), usize>,
    pub stop_tracking_modal: Callback<usize>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ModalTrackingContextProvider(props: &Props) -> Html {
    let tracked_modal_ids_handle = use_reducer(TrackedModalIds::default);
    let active_modal_id = tracked_modal_ids_handle.last().unwrap_or_default();

    let newest_id = use_mut_ref(|| 0);

    let start_tracking_modal = use_memo(
        tracked_modal_ids_handle.clone(),
        move |tracked_modal_ids_handle| {
            let tracked_modal_ids_handle = tracked_modal_ids_handle.clone();
            Callback::from(move |()| {
                *newest_id.borrow_mut() += 1;
                let id = *newest_id.borrow_mut();

                tracked_modal_ids_handle.dispatch(Action::StartTracking(id));

                id
            })
        },
    );

    let stop_tracking_modal = use_memo(tracked_modal_ids_handle, move |tracked_modal_ids_handle| {
        let tracked_modal_ids_handle = tracked_modal_ids_handle.clone();
        Callback::from(move |id: usize| {
            tracked_modal_ids_handle.dispatch(Action::StopTracking(id));
        })
    });

    let context = ModalTrackingContext {
        active_modal_id,
        start_tracking_modal: start_tracking_modal.deref().clone(),
        stop_tracking_modal: stop_tracking_modal.deref().clone(),
    };

    html! {
        <ContextProvider<ModalTrackingContext> {context}>
            {props.children.clone()}
        </ContextProvider<ModalTrackingContext>>
    }
}
