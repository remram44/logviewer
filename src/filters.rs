use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
#[cfg(feature = "json")]
use serde_json::Value;

pub enum Operation {
    If(Condition, Vec<Operation>, Vec<Operation>),
    Set(String, Expression),
    ColorBy(Expression),
    SkipRecord,
}

pub enum Expression {
    Record,
    Var(String),
    LastVarValue(String),
    Constant(String),
}

pub enum Condition {
    Match(Expression, Pattern),
}

pub struct Pattern {
    pub regex: String,
    pub compiled: Regex,
    pub groups: Vec<String>,
    all_groups: Vec<Option<String>>,
}

pub struct View(pub Vec<Operation>);

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
            Condition::Match(expression, pattern) => {
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
            if let Operation::If(condition, then_ops, else_ops) = &else_ops[0] {
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
            Operation::If(condition, then_ops, else_ops) => {
                idt(f, indent)?;
                write!(f, "IF ")?;
                self.print_if_branch(f, indent, condition, then_ops, else_ops)?;
            }
            Operation::Set(target, expression) => {
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
    #[cfg(feature = "json")]
    pub fn from_json(json: &Value) -> Result<View, &'static str> {
        crate::json::read_view(json)
    }

    #[cfg(feature = "json")]
    pub fn to_json(&self) -> Value {
        todo!() // JSON
    }

    pub fn print(
        &self,
        f: &mut std::fmt::Formatter,
        indent: usize,
    ) -> std::fmt::Result {
        for operation in &self.0 {
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
