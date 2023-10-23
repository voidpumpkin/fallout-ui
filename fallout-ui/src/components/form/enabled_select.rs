use serde::Deserialize;
use serde::Serialize;
use strum::EnumIter;
use strum::IntoEnumIterator;
use yew::html::IntoPropValue;
use yew::prelude::*;

use crate::components::form::filterable_select::FilterableSelect;
use crate::utils::form::FieldControlProps;

#[derive(Debug, Clone, PartialEq, strum::Display, EnumIter, Eq, Serialize, Deserialize)]
pub enum IsEnabled {
    Enabled,
    Disabled,
}

impl Default for IsEnabled {
    fn default() -> Self {
        bool::default().into()
    }
}

impl From<bool> for IsEnabled {
    fn from(value: bool) -> Self {
        if value {
            IsEnabled::Enabled
        } else {
            IsEnabled::Disabled
        }
    }
}

impl From<IsEnabled> for bool {
    fn from(val: IsEnabled) -> Self {
        match val {
            IsEnabled::Enabled => true,
            IsEnabled::Disabled => false,
        }
    }
}

impl IntoPropValue<String> for IsEnabled {
    fn into_prop_value(self) -> String {
        self.to_string()
    }
}

lazy_static! {
    pub static ref OPTIONS: Vec<IsEnabled> = IsEnabled::iter().collect();
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub field_control_props: FieldControlProps<IsEnabled>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub container_class: Classes,
}

#[function_component]
pub fn EnabledSelect(props: &Props) -> Html {
    let Props {
        label,
        field_control_props,
        required,
        disabled,
        container_class,
    } = props.clone();

    html! {
        <FilterableSelect<IsEnabled>
          {label}
          options={OPTIONS.clone()}
          {field_control_props}
          {required}
          {disabled}
          {container_class}
        />
    }
}
