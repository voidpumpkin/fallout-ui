use std::rc::Rc;

use chrono::Utc;
use gloo::timers::callback::Interval;
use yew::prelude::*;

struct NowDateTimeReducer {
    now: String,
}

impl Default for NowDateTimeReducer {
    fn default() -> Self {
        let date_time = Utc::now();
        // %F - 2001-07-08
        // %R - 00:34
        let now = date_time.format("%FT%RZ").to_string();
        Self { now }
    }
}

impl Reducible for NowDateTimeReducer {
    type Action = ();

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
        Self::default().into()
    }
}

#[hook]
pub fn use_date_time_clock() -> String {
    let now_reducer_handle = use_reducer(NowDateTimeReducer::default);
    let now = now_reducer_handle.now.clone();

    use_effect_with((), move |_| {
        now_reducer_handle.dispatch(());

        let interval = Interval::new(1_000, move || now_reducer_handle.dispatch(()));

        move || drop(interval)
    });

    now
}
