use regex::Regex;
#[cfg(feature = "json")]
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt::Debug;

#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub enum Operation {
    If {
        condition: Condition,
        #[cfg_attr(feature = "json", serde(rename = "then"))]
        then_ops: Vec<Operation>,
        #[cfg_attr(feature = "json", serde(rename = "else"))]
        else_ops: Vec<Operation>,
    },
    Set {
        target: String,
        expression: Expression,
    },
    ColorBy(Expression),
    SkipRecord,
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub enum Expression {
    Record,
    Var(String),
    LastVarValue(String),
    Constant(String),
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
pub enum Condition {
    Match {
        expression: Expression,
        pattern: Pattern,
    },
}

pub struct Pattern {
    pub regex: String,
    pub compiled: Regex,
    pub groups: Vec<String>,
    all_groups: Vec<Option<String>>,
}

#[cfg(feature = "json")]
impl serde::ser::Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer
    {
        serializer.serialize_str(&self.regex)
    }
}

#[cfg(feature = "json")]
impl<'d> serde::de::Deserialize<'d> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'d>,
    {
        struct RegexVisitor;

        impl<'d> serde::de::Visitor<'d> for RegexVisitor {
            type Value = String;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("regex")
            }

            fn visit_str<E>(self, value: &str) -> Result<String, E>
            where
                E: serde::de::Error
            {
                Ok(value.to_owned())
            }

            fn visit_string<E>(self, value: String) -> Result<String, E>
            where
                E: serde::de::Error
            {
                Ok(value)
            }
        }

        let regex = deserializer.deserialize_string(RegexVisitor)?;
        Ok(Pattern::new(regex))
    }
}

#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct View {
    pub operations: Vec<Operation>,
}

impl Pattern {
    pub fn new(regex: String) -> Pattern {
        let compiled = Regex::new(&regex).expect("Invalid regex");
        let all_groups: Vec<Option<String>> = compiled
            .capture_names() // Option<&str>
            .map(|v: Option<&str>| v.map(ToOwned::to_owned)) // Option<String>
            .collect();
        let groups = all_groups.iter()
            .filter_map(|v| v.as_ref().cloned())
            .collect();
        Pattern {
            regex,
            compiled,
            groups,
            all_groups,
        }
    }

    pub fn match_string(&self, string: &String) -> Option<HashMap<String, String>> {
        match self.compiled.captures(string) {
            Some(m) => {
                let mut map: HashMap<String, String> = HashMap::new();
                for (value, key) in m.iter().zip(&self.all_groups) {
                    match (key, value) {
                        (Some(key), Some(value)) => {
                            map.insert(
                                key.to_owned(),
                                value.as_str().to_owned(),
                            );
                        }
                        _ => {}
                    }
                }
                Some(map)
            }
            None => None,
        }
    }
}

fn idt(f: &mut std::fmt::Formatter, indent: usize) -> std::fmt::Result {
    for _ in 0..indent {
        write!(f, "  ")?;
    }
    Ok(())
}

impl Expression {
    pub fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Record => write!(f, "record"),
            Expression::Var(name) => write!(f, "variable {}", name),
            Expression::LastVarValue(name) => write!(f, "last value of variable {}", name),
            Expression::Constant(value) => write!(f, "{:?}", value),
        }
    }
}

impl Operation {
    fn print_if_branch(
        &self,
        f: &mut std::fmt::Formatter,
        indent: usize,
        condition: &Condition,
        then_ops: &Vec<Operation>,
        else_ops: &Vec<Operation>,
    ) -> std::fmt::Result {
        match condition {
            Condition::Match { expression, pattern } => {
                expression.print(f)?;
                write!(f, " match \"{}\"", pattern.regex)?;
            }
        }
        write!(f, "\n")?;
        if then_ops.is_empty() {
            idt(f, indent + 1)?;
            write!(f, "NOTHING\n")?;
        } else {
            for op in then_ops {
                op.print(f, indent + 1)?;
            }
        }
        let else_if = if else_ops.len() == 1 {
            if let Operation::If { condition, then_ops, else_ops } = &else_ops[0] {
                idt(f, indent)?;
                write!(f, "ELIF ")?;
                self.print_if_branch(f, indent, condition, then_ops, else_ops)?;
                true
            } else {
                false
            }
        } else {
            false
        };
        if !else_ops.is_empty() && !else_if {
            idt(f, indent)?;
            write!(f, "ELSE\n")?;
            for op in else_ops {
                op.print(f, indent + 1)?;
            }
        }
        Ok(())
    }

    pub fn print(
        &self,
        f: &mut std::fmt::Formatter,
        indent: usize,
    ) -> std::fmt::Result {
        match self {
            Operation::If { condition, then_ops, else_ops } => {
                idt(f, indent)?;
                write!(f, "IF ")?;
                self.print_if_branch(f, indent, condition, then_ops, else_ops)?;
            }
            Operation::Set { target, expression } => {
                idt(f, indent)?;
                write!(f, "SET {} = ", target)?;
                expression.print(f)?;
                write!(f, "\n")?;
            }
            Operation::ColorBy(expression) => {
                idt(f, indent)?;
                write!(f, "COLOR-BY ")?;
                expression.print(f)?;
                write!(f, "\n")?;
            }
            Operation::SkipRecord => {
                idt(f, indent)?;
                write!(f, "SKIP\n")?;
            }
        }
        Ok(())
    }
}

impl View {
    pub fn print(
        &self,
        f: &mut std::fmt::Formatter,
        indent: usize,
    ) -> std::fmt::Result {
        for operation in &self.operations {
            operation.print(f, indent)?;
        }
        Ok(())
    }
}

impl Debug for View {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "View [\n")?;
        self.print(f, 1)?;
        write!(f, "]")?;
        Ok(())
    }
}
