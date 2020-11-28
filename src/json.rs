use crate::filters::{Condition, Expression, Operation, Pattern, View};
use serde_json::Value;

type Map = serde_json::Map<String, Value>;

fn read_expression(json: &Map) -> Result<Expression, &'static str> {
    if json.len() != 1 {
        return Err("Invalid expression");
    }
    let (key, value) = match json.iter().next() {
        None => panic!(), // Can't happen, len() == 1
        Some(e) => e,
    };
    if key == "record" {
        match value {
            Value::Object(v) => {
                if v.len() > 0 {
                    return Err("Unexpected keys for record");
                }
            }
            _ => return Err("Expected object for record"),
        }
        Ok(Expression::Record)
    } else if key == "variable" {
        match value {
            Value::String(name) => Ok(Expression::Var(name.to_owned())),
            _ => return Err("Expected string for variable"),
        }
    } else if key == "lastVariableValue" {
        match value {
            Value::String(name) => Ok(Expression::LastVarValue(name.to_owned())),
            _ => return Err("Expected string for lastVariableValue"),
        }
    } else if key == "constant" {
        match value {
            Value::String(c) => Ok(Expression::Constant(c.to_owned())),
            _ => return Err("Expected string for constant"),
        }
    } else {
        return Err("Unknown expression");
    }
}

fn read_operation(json: &Value) -> Result<Operation, &'static str> {
    let json = match json {
        Value::Object(v) => v,
        _ => return Err("Expected object"),
    };
    if json.len() != 1 {
        return Err("Invalid operation");
    }
    match json.iter().next() {
        None => panic!(), // Can't happen, len() == 1
        Some((key, Value::Object(value))) => {
            if key == "if" {
                let mut condition = None;
                if let Some(match_) = value.get("match") {
                    let match_ = match match_ {
                        Value::Object(v) => v,
                        _ => return Err("Expected object for match"),
                    };
                    let expression = match match_.get("expression") {
                        Some(Value::Object(e)) => read_expression(e)?,
                        _ => return Err("Expected object expression in match"),
                    };
                    let pattern = match match_.get("pattern") {
                        Some(Value::String(p)) => Pattern::new(p.to_owned()),
                        Some(_) => return Err("Expected string for pattern"),
                        None => return Err("Missing pattern in match"),
                    };
                    for key in match_.keys() {
                        if key != "expression" && key != "pattern" {
                            return Err("Unexpected keys in match");
                        }
                    }
                    condition = Some(Condition::Match(expression, pattern));
                }
                let condition = match condition {
                    Some(c) => c,
                    None => return Err("Unknown condition for If"),
                };
                let mut then_ops = Vec::new();
                match value.get("then") {
                    Some(Value::Array(ops)) => {
                        for op in ops {
                            then_ops.push(read_operation(op)?);
                        }
                    }
                    Some(_) => return Err("Expected array for then operations"),
                    None => {}
                }
                let mut else_ops = Vec::new();
                match value.get("else") {
                    Some(Value::Array(ops)) => {
                        for op in ops {
                            else_ops.push(read_operation(op)?);
                        }
                    }
                    Some(_) => return Err("Expected array for else operations"),
                    None => {}
                }
                for key in value.keys() {
                    if key != "match" && key != "then" && key != "else" {
                        return Err("Unexpected keys in set");
                    }
                }
                Ok(Operation::If(condition, then_ops, else_ops))
            } else if key == "set" {
                let target = match value.get("target") {
                    Some(Value::String(name)) => name.to_owned(),
                    _ => return Err("Expected string target for set"),
                };
                let expression = match value.get("expression") {
                    Some(Value::Object(v)) => read_expression(v)?,
                    Some(_) => return Err("Expected object expression for set"),
                    None => Expression::Constant("".to_owned()),
                };
                for key in value.keys() {
                    if key != "target" && key != "expression" {
                        return Err("Unexpected keys in set");
                    }
                }
                Ok(Operation::Set(target, expression))
            } else if key == "colorBy" {
                let expression = read_expression(value)?;
                Ok(Operation::ColorBy(expression))
            } else if key == "skip" {
                if value.len() > 0 {
                    return Err("Unexpected keys for skip");
                }
                Ok(Operation::SkipRecord)
            } else {
                return Err("Unknown operation");
            }
        }
        Some((_, _)) => panic!(),
    }
}

pub fn read_view(json: &Value) -> Result<View, &'static str> {
    let json = match json {
        Value::Object(v) => v,
        _ => return Err("Expected object"),
    };
    for k in json.keys() {
        if k != "operations" {
            return Err("Unexpected keys");
        }
    }
    let operations_json = match json.get("operations") {
        Some(Value::Array(v)) => v,
        Some(_) => return Err("Expected array"),
        None => return Err("Expected key \"operations\""),
    };
    let mut operations = Vec::with_capacity(operations_json.len());
    for operation_json in operations_json {
        operations.push(read_operation(operation_json)?);
    }
    Ok(View(operations))
}

#[test]
fn test_deserialize() {
    let file = std::fs::File::open("test.json").unwrap();
    let json = serde_json::from_reader(file).unwrap();
    let view = View::from_json(&json).unwrap();
    println!("{:?}", view);
}
