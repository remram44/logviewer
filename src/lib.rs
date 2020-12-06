pub mod filters;
mod process;
pub mod readers;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use process::{FilteredLogIterator, process};

#[cfg_attr(feature = "json", derive(serde_derive::Serialize))]
pub enum Color {
    #[cfg_attr(feature = "json", serde(rename = "default"))]
    Default,
    #[cfg_attr(feature = "json", serde(rename = "fixed"))]
    Fixed {
        color: String,
    },
    #[cfg_attr(feature = "json", serde(rename = "fromValue"))]
    FromValue {
        value: String,
    },
}

#[cfg_attr(feature = "json", derive(serde_derive::Serialize))]
pub struct Record {
    pub text: String,
    pub variables: HashMap<String, String>,
    pub color: Color,
}

impl Record {
    fn new(text: String) -> Record {
        Record {
            text,
            variables: HashMap::new(),
            color: Color::Default,
        }
    }
}
