use std::rc::Rc;

use yew::prelude::*;

enum Action {
    Increment,
    Decrement,
}

#[derive(Default, Clone)]
struct CounterState {
    counter: i64,
}

impl Reducible for CounterState {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let counter = match action {
            Action::Increment => self.counter + 1,
            Action::Decrement => self.counter - 1,
        };

        CounterState { counter }.into()
    }
}

#[derive(Clone)]
pub struct UseLoadingCountHandle {
    counter_state_handle: UseReducerHandle<CounterState>,
}

impl UseLoadingCountHandle {
    pub fn is_loading(&self) -> bool {
        self.counter_state_handle.counter > 0
    }

    pub fn increment(&self) {
        self.counter_state_handle.dispatch(Action::Increment)
    }

    pub fn decrement(&self) {
        self.counter_state_handle.dispatch(Action::Decrement)
    }
}

/// Used when it is possible to have multiple loading states at once
#[hook]
pub fn use_loading_count() -> UseLoadingCountHandle {
    let counter_state_handle = use_reducer(CounterState::default);
    UseLoadingCountHandle {
        counter_state_handle,
    }
}
