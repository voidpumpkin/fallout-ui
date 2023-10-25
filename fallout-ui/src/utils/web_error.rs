use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use validator::ValidationErrors;

use crate::utils::display_validation_errs;
use crate::utils::js::JsError;

pub type WebResult<T> = Result<T, WebError>;

#[derive(Debug, Clone)]
pub enum WebErrorKind {
    JsError(JsError),
    LogicError,
    Validation(HashMap<String, String>),
}

/// Error that covers all errors that happen in our frontend aka web
#[derive(Debug)]
pub struct WebError {
    pub details_for_developer: Option<String>,
    pub kind: WebErrorKind,
}

impl Display for WebError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            WebError {
                kind: WebErrorKind::Validation(errs),
                ..
            } => display_validation_errs(f, errs)?,
            WebError {
                kind: WebErrorKind::JsError(_),
                ..
            } => write!(
                f,
                "We ran into a problem, try refreshing the page or contact our support"
            )?,
            WebError {
                kind: WebErrorKind::LogicError,
                ..
            } => write!(f, "We ran into a problem, contact our support")?,
        };
        Ok(())
    }
}

impl Error for WebError {}

pub fn web_err_logic<T>(details_for_developer: T) -> WebError
where
    T: ToString,
{
    WebError {
        kind: WebErrorKind::LogicError,
        details_for_developer: Some(details_for_developer.to_string()),
    }
}

pub fn web_err_validation<T>(errs: T) -> WebError
where
    T: Into<HashMap<String, String>>,
{
    WebError {
        kind: WebErrorKind::Validation(errs.into()),
        details_for_developer: None,
    }
}

pub fn web_err_validation_only_message<T>(msg: T) -> WebError
where
    T: Into<String>,
{
    let mut errs = HashMap::new();
    errs.insert(String::default(), msg.into());

    WebError {
        kind: WebErrorKind::Validation(errs),
        details_for_developer: None,
    }
}

pub fn web_err_js<T>(err: T) -> WebError
where
    T: Into<JsError>,
{
    WebError {
        kind: WebErrorKind::JsError(err.into()),
        details_for_developer: None,
    }
}

impl From<JsError> for WebError {
    fn from(err: JsError) -> Self {
        web_err_js(err)
    }
}

impl From<ValidationErrors> for WebError {
    fn from(validator_errors: ValidationErrors) -> Self {
        let errs: HashMap<String, String> = validator_errors
            .field_errors()
            .iter()
            .map(|(field, errors)| {
                (
                    field.to_string(),
                    errors
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(", "),
                )
            })
            .collect();
        web_err_validation(errs)
    }
}
