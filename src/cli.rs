use clap::{App, Arg, SubCommand, crate_version};
use std::fs::File;
use std::io::{Write, stdout};
use std::process;

use logviewer::process;
use logviewer::readers::LogFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("logviewer")
        .about("Log Viewer")
        .version(crate_version!())
        .author("Remi Rampin <remirampin@gmail.com>")
        .subcommand(SubCommand::with_name("process")
                    .about("Process a log file according to a view (JSON) and \
                            output records (JSON lines)")
                    .arg(Arg::with_name("VIEW")
                         .required(true)
                         .help("View definition (JSON file)"))
                    .arg(Arg::with_name("LOG")
                         .required(true)
                         .help("Log file")));
    #[cfg(feature = "web")]
    let app = app
        .subcommand(SubCommand::with_name("web")
                    .about("Start a local webserver to analyze logs")
                    .arg(Arg::with_name("LOG")
                         .required(true)
                         .help("Log file")));

    let matches = app.get_matches();

    let (command, matches) = match matches.subcommand() {
        (_, None) => {
            eprintln!("No command specified.");
            process::exit(1);
        }
        (command, Some(matches)) => (command, matches),
    };

    match command {
        "process" => {
            let log_file = {
                let path = matches.value_of_os("LOG").unwrap();
                LogFile::open(path)?
            };
            let view = {
                let path = matches.value_of_os("VIEW").unwrap();
                let file = File::open(path)?;
                serde_json::from_reader(file)?
            };

            // Process records
            let out = stdout();
            let mut out = out.lock();
            for record in process(log_file, view) {
                let record = record?;
                serde_json::to_writer(&mut out, &record)?;
                write!(out, "\n")?;
            }

            // TODO: Allocate concrete colors for FromValue colors, print with
            // ANSI colors in terminal
        }
        #[cfg(feature = "web")]
        "web" => {
            let mut runtime = tokio::runtime::Builder::new()
                .basic_scheduler()
                .enable_all()
                .build()
                .unwrap();
            runtime.block_on(
                logviewer::web::serve([127, 0, 0, 1].into(), 8000),
            );
        }
        _ => panic!("Missing code for command {}", command),
    }
    Ok(())
}
