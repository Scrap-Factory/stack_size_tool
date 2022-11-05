use std::fs;

use error_stack::{IntoReport, Result, ResultExt};
use json_comments::StripComments;
use serde::Deserialize;

use crate::errors::FromFileParseError;

pub trait FromFile<T: for<'a> Deserialize<'a>> {
    fn from_file(path: &str) -> Result<T, FromFileParseError> {
        let content = fs::read_to_string(path)
            .into_report()
            .attach_printable_lazy(|| format!("Could not read file {path}"))
            .change_context(FromFileParseError)?;

        let content = StripComments::new(content.as_bytes());

        let json = serde_json::from_reader(content)
            .into_report()
            .attach_printable_lazy(|| format!("Could not parse file {path:#?}"))
            .change_context(FromFileParseError)?;

        Ok(json)
    }
}
