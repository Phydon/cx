use clap::{Arg, ArgAction, Command};
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::{error, warn};
use owo_colors::colored::*;

use std::{
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    process,
};

fn main() {
    // handle Ctrl+C
    ctrlc::set_handler(move || {
        println!(
            "{} {} {} {}",
            "Received Ctrl-C!".bold().red(),
            "🤬",
            "Exit program!".bold().red(),
            "☠",
        );
        process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");

    // get config dir
    let config_dir = check_create_config_dir().unwrap_or_else(|err| {
        error!("Unable to find or create a config directory: {err}");
        process::exit(1);
    });

    // initialize the logger
    let _logger = Logger::try_with_str("info") // log warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(
            FileSpec::default()
                .directory(&config_dir)
                .suppress_timestamp(),
        ) // change directory for logs, no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Info) // print infos, warnings and errors also to the console
        .start()
        .unwrap();

    // handle arguments
    let matches = countx().get_matches();
    let chars_flag = matches.get_flag("chars");
    let lines_flag = matches.get_flag("lines");
    let word_flag = matches.get_flag("word");

    if let Some(_) = matches.subcommand_matches("log") {
        if let Ok(logs) = show_log_file(&config_dir) {
            println!("{}", "Available logs:".bold().yellow());
            println!("{}", logs);
        } else {
            error!("Unable to read logs");
            process::exit(1);
        }
    } else {
        let mut content = String::new();

        if let Some(arg) = matches.get_one::<String>("arg") {
            let path = Path::new(&arg);

            if !path.exists() {
                warn!("Path '{}' doesn`t exist", path.display());
                process::exit(0);
            }

            if !path.is_file() {
                warn!("Path '{}' is not a file", path.display());
                process::exit(0);
            }

            let file_content = fs::read_to_string(path).unwrap_or_else(|err| {
                match err.kind() {
                    io::ErrorKind::InvalidData => {
                        warn!("Path \'{}\' contains invalid data: {}", path.display(), err)
                    }
                    io::ErrorKind::NotFound => {
                        warn!("Path \'{}\' not found: {}", path.display(), err);
                    }
                    io::ErrorKind::PermissionDenied => {
                        warn!(
                            "Missing permission to read path \'{}\': {}",
                            path.display(),
                            err
                        )
                    }
                    _ => {
                        error!(
                            "Failed to access path: \'{}\'\nUnexpected error occurred: {}",
                            path.display(),
                            err
                        )
                    }
                }
                process::exit(0);
            });

            content.push_str(&file_content);
        } else {
            // read input from pipe
            let input = read_pipe();

            content.push_str(&input);
        }

        if word_flag {
            todo!();
        } else if lines_flag {
            let mut line_count = 0;
            for _ in content.lines() {
                line_count += 1;
            }

            println!("{}", line_count.blue());
        } else if chars_flag {
            todo!();
        } else {
            todo!("Count words as default");
        }
    }
}

// build cli
fn countx() -> Command {
    Command::new("cx")
        .bin_name("cx")
        .before_help(format!(
            "{}\n{}",
            "CX".bold().truecolor(250, 0, 104),
            "Leann Phydon <leann.phydon@gmail.com>".italic().dimmed()
        ))
        .about("Count X")
        .before_long_help(format!(
            "{}\n{}",
            "CX".bold().truecolor(250, 0, 104),
            "Leann Phydon <leann.phydon@gmail.com>".italic().dimmed()
        ))
        // TODO update version
        .version("1.0.0")
        .author("Leann Phydon <leann.phydon@gmail.com>")
        .arg(
            Arg::new("arg")
                .help("The filepath to work with")
                .action(ArgAction::Set)
                .num_args(1)
                .value_name("PATH"),
        )
        .arg(
            Arg::new("chars")
                .short('c')
                .long("chars")
                .help("Count all chars")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("Count all lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("word")
                .short('w')
                .long("word")
                .help("Count all words")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("log")
                .short_flag('L')
                .long_flag("log")
                .about("Show content of the log file"),
        )
}

fn read_pipe() -> String {
    let mut input = io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");

    // TODO possible error here?
    // TODO if last char is '\n' it will get removed
    let _ = input.pop();

    input.trim().to_string()
}

fn check_create_config_dir() -> io::Result<PathBuf> {
    let mut new_dir = PathBuf::new();
    match dirs::config_dir() {
        Some(config_dir) => {
            new_dir.push(config_dir);
            new_dir.push("cx");
            if !new_dir.as_path().exists() {
                fs::create_dir(&new_dir)?;
            }
        }
        None => {
            error!("Unable to find config directory");
        }
    }

    Ok(new_dir)
}

fn show_log_file(config_dir: &PathBuf) -> io::Result<String> {
    let log_path = Path::new(&config_dir).join("cx.log");
    match log_path.try_exists()? {
        true => {
            return Ok(format!(
                "{} {}\n{}",
                "Log location:".italic().dimmed(),
                &log_path.display(),
                fs::read_to_string(&log_path)?
            ));
        }
        false => {
            return Ok(format!(
                "{} {}",
                "No log file found:"
                    .truecolor(250, 0, 104)
                    .bold()
                    .to_string(),
                log_path.display()
            ))
        }
    }
}
