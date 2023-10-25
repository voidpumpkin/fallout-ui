use yew::prelude::*;

use crate::components::form::base::base_filterable_nullable_select::BaseFilterableNullableSelect;
use crate::utils::form::FieldControlProps;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub field_control_props: FieldControlProps<Option<String>>,
    pub mcc_codes: MCCCodes,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub container_class: Classes,
}

pub type MCCCodes = Vec<[String; 2]>;

#[function_component]
pub fn MCCInput(props: &Props) -> Html {
    let Props {
        label,
        field_control_props:
            FieldControlProps {
                value,
                error,
                onchange,
                onblur,
            },
        mcc_codes,
        required,
        disabled,
        container_class,
    } = props.clone();

    let mcc_codes_rc = use_memo(mcc_codes, move |mcc_codes| {
        mcc_codes
            .iter()
            .map(|v| v[1].clone())
            .collect::<Vec<String>>()
    });
    let mcc_codes: Vec<String> = (*mcc_codes_rc).clone();

    let handle_select = {
        Callback::from(move |value: Option<String>| {
            onchange.emit(value);
        })
    };

    let option_html: Callback<String, Html> = Callback::from(|option: String| -> Html {
        let (code, rest) = option.split_at(4);
        html! {
            <>
                <span class="font-mono">{code}</span>
                {rest}
            </>
        }
    });

    html! {
        <BaseFilterableNullableSelect
          {label}
          options={mcc_codes}
          {value}
          on_select={handle_select}
          {error}
          {onblur}
          {required}
          {option_html}
          {disabled}
          {container_class}
        />
    }
}
