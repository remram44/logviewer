pub mod filters;
mod process;
pub mod readers;

use filters::{View, Operation, Expression, Condition, Pattern};
use process::process;
use readers::LogFile;

#[test]
fn test_filter() -> Result<(), Box<dyn std::error::Error>> {
    let file = LogFile::open("test.log")?;
    let view = View(vec![
        // If: has timestamp
        Operation::If(
            Condition::Match(
                Expression::Record,
                Pattern {
                    regex: "^(?P<time>[0-9TZ-]+) (?P<message>.*)$".to_owned(),
                    groups: vec!["time".to_owned(), "message".to_owned()],
                },
            ),
            // Hash timestamp, then
            vec![
                // If, message is HTTP access
                Operation::If(
                    Condition::Match(
                        Expression::Var("message".to_owned()),
                        Pattern {
                            regex: "^(?P<client>[0-9]+(\\.[0-9]+){3}) ([^ ]+ ){2}\\[.+\\] \"(?P<vhost>[^\"]+)\"".to_owned(),
                            groups: vec![],
                        },
                    ),
                    // HTTP access, then
                    vec![
                        Operation::Set(
                            "service".to_owned(),
                            Expression::Constant("frontend".to_owned()),
                        ),
                    ],
                    // HTTP access, else
                    vec![
                        // If, message has service name
                        Operation::If(
                            Condition::Match(
                                Expression::Var("message".to_owned()),
                                Pattern {
                                    regex: "^service=(?P<service>[^ ]+) (?P<message>.*)$".to_owned(),
                                    groups: vec!["service".to_owned()],
                                },
                            ),
                            vec![],
                            vec![],
                        ),
                    ],
                ),
                Operation::ColorBy(
                    Expression::Var("service".to_owned())
                ),
            ],
            // Has timestamp, else
            vec![
                Operation::Set(
                    "time".to_owned(),
                    Expression::LastVarValue("time".to_owned()),
                ),
            ],
        ),
        // If: is error
        Operation::If(
            Condition::Match(
                Expression::Record,
                Pattern {
                    regex: "\\bERROR\\b".to_owned(),
                    groups: vec![],
                },
            ),
            // Is error, then
            vec![
                Operation::Set(
                    "error".to_owned(),
                    Expression::Constant("".to_owned()),
                ),
            ],
            // Is error, else
            vec![
                Operation::If(
                    Condition::Match(
                        Expression::Record,
                        Pattern {
                            regex: "\\bDEBUG\\b".to_owned(),
                            groups: vec![],
                        },
                    ),
                    vec![
                        Operation::SkipRecord,
                    ],
                    vec![],
                ),
            ],
        ),
    ]);

    // Process records
    for record in process(file, view) {
    }
    Ok(())
}
