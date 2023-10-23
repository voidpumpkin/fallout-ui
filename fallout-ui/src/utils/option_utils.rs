pub fn if_changed_then_some<T: PartialEq>(new_value: T, previous_value: &T) -> Option<T> {
    if &new_value == previous_value {
        None
    } else {
        Some(new_value)
    }
}
