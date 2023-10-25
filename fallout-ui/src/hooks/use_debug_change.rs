use std::fmt::Debug;
use std::rc::Rc;

use clone_on_capture::clone_on_capture;
use yew::prelude::*;

use super::use_is_first_mount;

/// will print to console when provided value changes
#[macro_export]
macro_rules! use_debug_change {
    ($var:expr) => {
        $crate::hooks::use_debug_change_inner(
            format!("{}:{}: {}", file!(), line!(), stringify!($var)),
            $var,
        )
    };
}

#[clone_on_capture]
#[hook]
pub fn use_debug_change_inner<T>(variable_name: String, changing_variable: T)
where
    T: Debug + PartialEq + 'static,
{
    let variable_name = Rc::new(variable_name);
    let changing_variable = Rc::new(changing_variable);

    let changing_variable_prev_ref = use_mut_ref(|| changing_variable.clone());
    // Update the ref each render so if it changes the newest value will be saved.
    let changing_variable_prev = changing_variable_prev_ref.replace(changing_variable.clone());

    let is_first_mount = use_is_first_mount();

    use_effect_with(
        changing_variable.clone(),
        move |changing_variable: &Rc<T>| {
            if !is_first_mount {
                log::debug!(
                    "{} updated: from {:?}, to {:?}",
                    variable_name,
                    *changing_variable_prev,
                    changing_variable
                );
            }
        },
    );
}
