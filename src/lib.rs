pub mod filters;
#[cfg(feature = "json")]
mod json;
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
                Pattern::new("^(?P<time>[0-9TZ:-]+) (?P<message>.*)$".to_owned()),
            ),
            // Hash timestamp, then
            vec![
                // If, message is HTTP access
                Operation::If(
                    Condition::Match(
                        Expression::Var("message".to_owned()),
                        Pattern::new("^(?P<client>[0-9]+(\\.[0-9]+){3}) ([^ ]+ ){2}\\[.+\\] \"(?P<vhost>[^\"]+)\"".to_owned()),
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
                                Pattern::new("^service=(?P<service>[^ ]+) (?P<message>.*)$".to_owned()),
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
                Pattern::new("\\bERROR\\b".to_owned()),
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
                        Pattern::new("\\bDEBUG\\b".to_owned()),
                    ),
                    vec![
                        Operation::SkipRecord,
                    ],
                    vec![],
                ),
            ],
        ),
    ]);

    // Pretty-print filters
    println!("{:?}", view);

    // Process records
    for record in process(file, view) {
        let record = record?;
        println!("{}", record.text);
        for (key, value) in &record.variables {
            println!("    {} = {:?}", key, value);
        }
        println!("");
    }
    Ok(())
}
