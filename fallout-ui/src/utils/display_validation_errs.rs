use std::collections::HashMap;
use std::fmt::Formatter;
use std::fmt::{self};

pub fn display_validation_errs(
    f: &mut Formatter<'_>,
    errs: &HashMap<String, String>,
) -> fmt::Result {
    for (field_name, err) in errs.iter() {
        writeln!(f, " {field_name}: {err}")?;
    }
    Ok(())
}
