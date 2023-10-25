use strum::EnumIter;
use strum::IntoEnumIterator;
use yew::prelude::*;

use crate::components::form::filterable_select::FilterableSelect;
use crate::hooks::use_memo_owned;
use crate::utils::form::FieldControlProps;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub field_control_props: FieldControlProps<Option<bool>>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Debug, Clone, PartialEq, strum::Display, EnumIter, Eq, Default)]
enum EnabledOrAll {
    #[default]
    All,
    Enabled,
    Disabled,
}

impl From<Option<bool>> for EnabledOrAll {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(true) => Self::Enabled,
            Some(false) => Self::Disabled,
            None => Self::All,
        }
    }
}

impl From<EnabledOrAll> for Option<bool> {
    fn from(value: EnabledOrAll) -> Self {
        match value {
            EnabledOrAll::Enabled => Some(true),
            EnabledOrAll::Disabled => Some(false),
            EnabledOrAll::All => None,
        }
    }
}

#[function_component]
pub fn EnabledOrAllSelect(props: &Props) -> Html {
    let Props {
        label,
        field_control_props:
            FieldControlProps {
                value,
                error,
                onchange,
                onblur,
            },
        required,
        disabled,
        class,
    } = props.clone();

    let options = use_memo_owned(|_| EnabledOrAll::iter().collect::<Vec<_>>(), ());
    let value: EnabledOrAll = use_memo_owned(|value| (*value).into(), value);

    let onchange = use_callback(onchange, move |selected_value: EnabledOrAll, onchange| {
        onchange.emit(selected_value.into());
    });

    html! {
        <FilterableSelect<EnabledOrAll>
          {label}
          {options}
          field_control_props={FieldControlProps {
            value,
            error,
            onchange,
            onblur,
            }}
          {required}
          {disabled}
          container_class={class}
        />
    }
}
