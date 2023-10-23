use std::collections::HashMap;
use std::hash::Hash;

use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct FieldControlProps<T>
where
    T: Clone + PartialEq + 'static,
{
    pub value: T,
    pub error: Option<String>,
    pub onchange: Callback<T>,
    pub onblur: Callback<()>,
}

impl<T> FieldControlProps<T>
where
    T: Clone + PartialEq + 'static,
{
    pub fn from_value(value: T) -> Self {
        FieldControlProps {
            value,
            error: Default::default(),
            onchange: Default::default(),
            onblur: Default::default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct DynamicFieldControlProps<T, K, V>
where
    K: PartialEq + Hash + Eq,
    T: Clone + PartialEq + 'static,
{
    pub value: T,
    pub error: HashMap<K, String>,
    pub onchange: Callback<(K, V)>,
    pub onblur: Callback<K>,
}

impl<T, K, V> DynamicFieldControlProps<T, K, V>
where
    K: PartialEq + Hash + Eq,
    T: Clone + PartialEq + 'static,
{
    pub fn from_value(value: T) -> Self {
        DynamicFieldControlProps {
            value,
            error: Default::default(),
            onchange: Default::default(),
            onblur: Default::default(),
        }
    }
}
