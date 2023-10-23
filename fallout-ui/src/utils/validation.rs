use std::borrow::Cow;
use std::str::FromStr;

use regex::Regex;
use uuid::Uuid;
use validator::ValidationError;

/// Used to trick `validator` into passing option to the check function
/// https://github.com/Keats/validator/blob/44cc91749c675985468e59e126d76465fc675fb5/README.md?plain=1#L55
pub type Optional<T> = Option<T>;

lazy_static! {
    pub static ref PHONE_REGEX: Regex = Regex::new(r"\+\d+").unwrap();
}

pub fn validate_uuid(data: &str) -> Result<(), ValidationError> {
    if !data.is_empty() && Uuid::from_str(data).is_err() {
        return Err(validation_error(
            "Please enter valid UUID. Example: a9114a3e-3fc6-4136-bbb8-761f207262ed",
        ));
    }
    Ok(())
}

pub fn validate_phone_number(data: &str) -> Result<(), ValidationError> {
    checks::check_string_not_empty(data)?;
    checks::check_trailing_spaces(data)?;
    checks::check_spaces(data)?;
    checks::check_phone(data)?;

    Ok(())
}

pub fn validate_email(data: &str) -> Result<(), ValidationError> {
    checks::check_string_not_empty(data)?;
    checks::check_trailing_spaces(data)?;
    checks::check_lowercase(data)?;
    checks::check_email(data)?;

    Ok(())
}

pub mod checks {
    use super::*;

    pub fn check_some<T>(data: &Option<T>) -> Result<(), ValidationError> {
        if data.is_none() {
            return Err(validation_error("This field is required"));
        }

        Ok(())
    }

    pub fn check_phone(data: &str) -> Result<(), ValidationError> {
        if !PHONE_REGEX.is_match(data) {
            return Err(validation_error(
                "please enter a valid phone number. Example: +37069943000",
            ));
        }

        let numbers = data.chars().filter(|c| c.is_numeric()).count();

        if numbers < 7 {
            return Err(validation_error("has to be at least 7 numbers long"));
        }

        if numbers > 15 {
            return Err(validation_error("has to be less than 16 characters long"));
        }

        Ok(())
    }

    pub fn check_email(data: &str) -> Result<(), ValidationError> {
        if !validator::validate_email(data) {
            return Err(validation_error("please enter a valid email address"));
        }

        Ok(())
    }

    pub fn check_string_not_empty(data: &str) -> Result<(), ValidationError> {
        if data.is_empty() {
            return Err(validation_error("this field is required"));
        }

        Ok(())
    }

    pub fn check_spaces(data: &str) -> Result<(), ValidationError> {
        if data.chars().any(|c| c.is_ascii_whitespace()) {
            return Err(validation_error("should not contain spaces"));
        }

        Ok(())
    }

    pub fn check_ascii(data: &str) -> Result<(), ValidationError> {
        if !data.is_ascii() {
            return Err(validation_error("can only contain ASCII characters"));
        }

        Ok(())
    }

    pub fn check_min_length(data: &str, len: usize) -> Result<(), ValidationError> {
        if data.chars().count() < len {
            return Err(validation_error_from_string(format!(
                "has to be at least {len} characters long"
            )));
        }

        Ok(())
    }

    pub fn check_max_length(data: &str, len: usize) -> Result<(), ValidationError> {
        if data.chars().count() > len {
            return Err(validation_error_from_string(format!(
                "has to be less than {len} characters long"
            )));
        }

        Ok(())
    }

    pub fn check_max_10_len(data: &str) -> Result<(), ValidationError> {
        check_max_length(data, 10)?;
        Ok(())
    }

    pub fn check_max_70_len(data: &str) -> Result<(), ValidationError> {
        check_max_length(data, 70)?;
        Ok(())
    }

    pub fn check_max_256_len(data: &str) -> Result<(), ValidationError> {
        check_max_length(data, 256)?;
        Ok(())
    }

    pub fn check_max_128_len(data: &str) -> Result<(), ValidationError> {
        check_max_length(data, 128)?;
        Ok(())
    }

    pub fn check_trailing_spaces(data: &str) -> Result<(), ValidationError> {
        if data.trim_start() != data {
            return Err(validation_error("should not start with spaces"));
        }

        if data.trim_end() != data {
            return Err(validation_error("should not end with spaces"));
        }

        Ok(())
    }

    pub fn check_lowercase(data: &str) -> Result<(), ValidationError> {
        if data.to_lowercase() != data {
            return Err(validation_error("must contain only lowercase letters"));
        }

        Ok(())
    }

    pub fn check_at_least_one_in_arr<T>(data: &Vec<T>) -> Result<(), ValidationError> {
        if data.is_empty() {
            return Err(validation_error("At least one must be selected"));
        }

        Ok(())
    }
}

pub fn validation_error(message: &'static str) -> ValidationError {
    let mut error = ValidationError::new("Input validation error");
    error.message = Some(Cow::from(message));
    error
}

pub fn validation_error_from_string(message: String) -> ValidationError {
    let mut error = ValidationError::new("Input validation error");
    error.message = Some(Cow::from(message));
    error
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_validation() {
        let phones = vec![
            "+6836234",
            "+441632960308",
            "+6791474582",
            "+865919990217",
            "+3502167524566",
            "+85234636717",
            "+37068679127",
            "+48692630017",
        ];

        for phone in phones {
            assert!(validate_phone_number(phone).is_ok());
        }
    }
}
