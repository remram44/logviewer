use std::collections::HashMap;
use std::io::{Error as IoError};

use crate::filters::{Condition, Expression, Operation, View};
use crate::readers::LogReader;

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

#[derive(Default)]
struct FilterInner {
    variables_last: HashMap<String, String>,
}

pub struct FilteredLogIterator<R: LogReader> {
    filter: FilterInner,
    reader: R,
    view: View,
}

impl FilterInner {
    fn set_variable(&mut self, record: &mut Record, key: String, value: String) {
        record.variables.insert(key.clone(), value.clone());
        self.variables_last.insert(key, value);
    }

    fn evaluate(&self, expression: &Expression, record: &Record) -> String {
        match expression {
            Expression::Record => record.text.to_owned(),
            Expression::Var(name) => record.variables.get(name).cloned().unwrap_or_default(),
            Expression::LastVarValue(name) => self.variables_last.get(name).cloned().unwrap_or_default(),
            Expression::Constant(value) => value.clone(),
        }
    }

    fn apply_operations(
        &mut self,
        record: &mut Record,
        operations: &[Operation],
    ) -> bool {
        for operation in operations {
            match operation {
                Operation::If { condition, then_ops, else_ops } => {
                    match condition {
                        Condition::Match { expression, pattern } => {
                            let value = self.evaluate(expression, &record);
                            if let Some(m) =  pattern.match_string(&value) {
                                for (key, value) in m {
                                    self.set_variable(record, key, value);
                                }
                                if !self.apply_operations(record, then_ops) {
                                    return false;
                                }
                            } else {
                                if !self.apply_operations(record, else_ops) {
                                    return false;
                                }
                            }
                        }
                    }
                }
                Operation::Set { target, expression } => {
                    let value = self.evaluate(expression, &record);
                    self.set_variable(record, target.to_owned(), value);
                }
                Operation::ColorBy(expression) => {
                    let value = self.evaluate(expression, &record);
                    record.color = Color::FromValue { value };
                }
                Operation::SkipRecord => {
                    return false;
                }
            }
        }
        true
    }
}

impl<R: LogReader> FilteredLogIterator<R> {
    fn next_triable(&mut self) -> Result<Option<Record>, IoError> {
        loop {
            // Read text from reader
            let text = match self.reader.read_record()? {
                Some(t) => t,
                None => return Ok(None),
            };
            let mut record = Record::new(text);

            // Apply filters
            if !self.filter.apply_operations(&mut record, &self.view.operations) {
                // Skipped
                continue
            }

            return Ok(Some(record));
        }
    }
}

impl<R: LogReader> Iterator for FilteredLogIterator<R> {
    type Item = Result<Record, IoError>;

    fn next(&mut self) -> Option<Result<Record, IoError>> {
        match self.next_triable() {
            Ok(Some(r)) => Some(Ok(r)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub fn process<R: LogReader>(reader: R, view: View) -> FilteredLogIterator<R> {
    FilteredLogIterator {
        filter: Default::default(),
        reader,
        view,
    }
}
