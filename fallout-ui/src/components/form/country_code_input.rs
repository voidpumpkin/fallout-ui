use std::str::FromStr;

use isocountry::CountryCode;
use serde::Serialize;
use yew::prelude::*;

use crate::components::form::base::base_filterable_nullable_select::BaseFilterableNullableSelect;
use crate::utils::form::FieldControlProps;
use crate::utils::toasts::notify_err;
use crate::utils::web_error::web_err_logic;
use crate::utils::web_error::WebError;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub field_control_props: FieldControlProps<Option<CountryCodeSelectOption>>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub container_class: Classes,
    #[prop_or_default]
    pub label_key: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct CountryCodeSelectOption(pub CountryCode);

impl std::str::FromStr for CountryCodeSelectOption {
    type Err = WebError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code_str = s.get(0..2).unwrap_or_default();
        let code = CountryCode::for_alpha2(code_str).map_err(web_err_logic)?;
        Ok(CountryCodeSelectOption(code))
    }
}

impl From<CountryCodeSelectOption> for CountryCode {
    fn from(val: CountryCodeSelectOption) -> Self {
        val.0
    }
}

impl ToString for CountryCodeSelectOption {
    fn to_string(&self) -> String {
        format!("{} - {}", self.0.alpha2(), self.0.name())
    }
}

#[function_component]
pub fn CountryCodeInput(props: &Props) -> Html {
    let Props {
        label,
        field_control_props:
            FieldControlProps {
                value,
                error,
                onchange,
                onblur,
            },
        container_class,
        required,
        disabled,
        label_key,
    } = props.clone();

    let value = value.map(|v| v.to_string()).unwrap_or_default();

    let country_codes_rc = use_memo((), move |_| {
        CountryCode::iter()
            .map(|c| CountryCodeSelectOption(*c).to_string())
            .collect::<Vec<String>>()
    });
    let country_codes: Vec<String> = (*country_codes_rc).clone();

    let option_html: Callback<String, Html> = Callback::from(|option: String| -> Html {
        let (code, rest) = option.split_at(2);
        html! {
            <>
                <span class="font-mono">{code}</span>
                {rest}
            </>
        }
    });

    let handle_select = {
        Callback::from(move |value: Option<String>| {
            let country_code = match value {
                Some(value) => match CountryCodeSelectOption::from_str(&value) {
                    Ok(ok) => Some(ok),
                    Err(err) => {
                        onchange.emit(None);
                        return notify_err(web_err_logic(err));
                    }
                },
                None => None,
            };
            onchange.emit(country_code);
        })
    };

    html! {
        <BaseFilterableNullableSelect
            {label_key}
            {label}
            options={country_codes}
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
