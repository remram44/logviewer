use crate::filters::{View, Operation, Expression, Condition, Pattern};
use crate::process;
use crate::readers::LogFile;

fn get_view() -> View {
    View {
        operations: vec![
            // If: has timestamp
            Operation::If {
                condition: Condition::Match {
                    expression: Expression::Record,
                    pattern: Pattern::new("^(?P<time>[0-9TZ:-]+) (?P<message>.*)$".to_owned()),
                },
                // Hash timestamp, then
                then_ops: vec![
                    // If, message is HTTP access
                    Operation::If {
                        condition: Condition::Match {
                            expression: Expression::Var("message".to_owned()),
                            pattern: Pattern::new("^(?P<client>[0-9]+(\\.[0-9]+){3}) ([^ ]+ ){2}\\[.+\\] \"(?P<vhost>[^\"]+)\"".to_owned()),
                        },
                        // HTTP access, then
                        then_ops: vec![
                            Operation::Set {
                                target: "service".to_owned(),
                                expression: Expression::Constant("frontend".to_owned()),
                            },
                        ],
                        // HTTP access, else
                        else_ops: vec![
                            // If, message has service name
                            Operation::If {
                                condition: Condition::Match {
                                    expression: Expression::Var("message".to_owned()),
                                    pattern: Pattern::new("^service=(?P<service>[^ ]+) (?P<message>.*)$".to_owned()),
                                },
                                then_ops: vec![],
                                else_ops: vec![],
                            },
                        ],
                    },
                    Operation::ColorBy(
                        Expression::Var("service".to_owned())
                    ),
                ],
                // Has timestamp, else
                else_ops: vec![
                    Operation::Set {
                        target: "time".to_owned(),
                        expression: Expression::LastVarValue("time".to_owned()),
                    },
                ],
            },
            // If: is error
            Operation::If {
                condition: Condition::Match {
                    expression: Expression::Record,
                    pattern: Pattern::new("\\bERROR\\b".to_owned()),
                },
                // Is error, then
                then_ops: vec![
                    Operation::Set {
                        target: "error".to_owned(),
                        expression: Expression::Constant("".to_owned()),
                    },
                ],
                // Is error, else
                else_ops: vec![
                    Operation::If {
                        condition: Condition::Match {
                            expression: Expression::Record,
                            pattern: Pattern::new("\\bDEBUG\\b".to_owned()),
                        },
                        then_ops: vec![
                            Operation::SkipRecord,
                        ],
                        else_ops: vec![],
                    },
                ],
            },
        ],
    }
}

#[test]
fn test_serialize() {
    let view = get_view();
    serde_json::to_writer_pretty(std::io::stdout(), &view).expect("Error serializing");
}

#[test]
fn test_print() {
    let view = get_view();
    println!("{:?}", view);
}

#[test]
fn test_process() {
    let file = LogFile::open("test.log").expect("Can't open test file test.log");
    let view = get_view();

    // Process records
    for record in process(file, view) {
        let record = record.expect("Error during processing");
        println!("{}", record.text);
        for (key, value) in &record.variables {
            println!("    {} = {:?}", key, value);
        }
        println!("");
    }
}
