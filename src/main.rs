// TODO use BigInt crate??
use clap::{Arg, ArgAction, Command};
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::{error, warn};
use owo_colors::colored::*;
use rayon::prelude::*;

use std::{
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    process,
};

fn main() {
    // handle Ctrl+C
    ctrlc::set_handler(move || {
        println!("{}", "Received Ctrl-C!".italic(),);
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
    let bytes_flag = matches.get_flag("bytes");
    let chars_flag = matches.get_flag("chars");
    let lines_flag = matches.get_flag("lines");
    let show_errors_flag = matches.get_flag("show-errors");
    let sum_flag = matches.get_flag("sum");
    let word_flag = matches.get_flag("words");

    if let Some(_) = matches.subcommand_matches("log") {
        if let Ok(logs) = show_log_file(&config_dir) {
            println!("{}", "Available logs:".bold().yellow());
            println!("{}", logs);
        } else {
            error!("Unable to read logs");
            process::exit(1);
        }
    } else if let Some(_) = matches.subcommand_matches("examples") {
        examples();
    } else {
        let mut content = String::new();

        if let Some(arg) = matches.get_one::<String>("arg") {
            // get filepath
            let path = Path::new(&arg);

            if !path.exists() {
                if show_errors_flag {
                    warn!("Path '{}' doesn`t exist", path.display());
                }
                process::exit(0);
            }

            if !path.is_file() {
                if show_errors_flag {
                    warn!("Path '{}' is not a file", path.display());
                }
                process::exit(0);
            }

            // read content from file
            let file_content = fs::read_to_string(path).unwrap_or_else(|err| {
                match err.kind() {
                    io::ErrorKind::InvalidData => {
                        if show_errors_flag {
                            warn!("Path \'{}\' contains invalid data: {}", path.display(), err)
                        }
                    }
                    io::ErrorKind::NotFound => {
                        if show_errors_flag {
                            warn!("Path \'{}\' not found: {}", path.display(), err);
                        }
                    }
                    io::ErrorKind::PermissionDenied => {
                        if show_errors_flag {
                            warn!(
                                "Missing permission to read path \'{}\': {}",
                                path.display(),
                                err
                            )
                        }
                    }
                    _ => {
                        if show_errors_flag {
                            error!(
                                "Failed to access path: \'{}\'\nUnexpected error occurred: {}",
                                path.display(),
                                err
                            )
                        }
                    }
                }
                process::exit(0);
            });

            content.push_str(&file_content);
        } else {
            // read input from stdin
            let input = read_stdin();
            content.push_str(&input);
        }

        let mut count: i128 = 0;
        if word_flag {
            count += count_words(content);
        } else if lines_flag {
            count += count_lines(content);
        } else if chars_flag {
            count += count_chars(content);
        } else if bytes_flag {
            count += count_bytes(content);
        } else if sum_flag {
            count += sum(content);
        } else {
            // count words by default
            count += count_words(content);
        }

        println!("{}", count);
    }
}

fn count_words(content: String) -> i128 {
    content.par_split_whitespace().count() as i128
}

fn count_lines(content: String) -> i128 {
    content.par_lines().count() as i128
}

fn count_chars(content: String) -> i128 {
    // TODO process in parallel
    let mut count = 0;
    content.split_whitespace().for_each(|word| {
        word.chars().for_each(|_| {
            count += 1;
        })
    });

    count

    // TODO FIXME
    // let mut count = 0;
    // content.par_split_whitespace().for_each(|word| {
    //     word.par_chars().for_each(|_| {
    //         count += 1;
    //     })
    // });
}

fn count_bytes(content: String) -> i128 {
    content.par_bytes().count() as i128
}

fn sum(content: String) -> i128 {
    content
        .par_split_whitespace()
        .filter_map(|x| match x.parse::<i128>() {
            Ok(n) => Some(n),
            _ => None,
        })
        .sum()
}

fn read_stdin() -> String {
    let mut input = io::stdin()
        .lock()
        .lines()
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");

    // TODO possible error here?
    // TODO if last char is '\n' it will get removed
    let _ = input.pop();

    input.trim().to_string()
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
        .version("1.2.3")
        .author("Leann Phydon <leann.phydon@gmail.com>")
        .arg(
            Arg::new("arg")
                .help("The filepath to work with")
                .long_help(format!(
                    "{}\n{}",
                    "The filepath to work with", "Reads stdin if left empty"
                ))
                .action(ArgAction::Set)
                .num_args(1)
                .value_name("PATH"),
        )
        .arg(
            Arg::new("bytes")
                .short('b')
                .long("bytes")
                .visible_alias("byte")
                .help("Count all bytes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("chars")
                .short('c')
                .long("chars")
                .visible_alias("char")
                .help("Count all chars")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .visible_alias("line")
                .help("Count all lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-errors")
                .short('S')
                .long("show-errors")
                .visible_alias("show-error")
                .help("Show errors (ignores errors by default)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sum")
                .short('s')
                .long("sum")
                .help("Sum up all integers")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .visible_alias("word")
                .help("Count all words")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("examples")
                .long_flag("examples")
                .about("Show examples"),
        )
        .subcommand(
            Command::new("log")
                .short_flag('L')
                .long_flag("log")
                .about("Show content of the log file"),
        )
}

fn examples() {
    println!("\n{}\n----------", "Example 1".bold());
    println!(
        r###"
$ cat example.txt
lorem ipsum wasd

$ cx example.txt --words
3

$cx example.txt --lines
1

$ cx example.txt --chars
14

$ cx example.txt --bytes
19
    "###
    );

    println!("\n{}\n----------", "Example 2".bold());
    println!(
        r###"
$ echo 'Some pipe input' | cx
3
    "###
    );

    println!("\n{}\n----------", "Example 3".bold());
    println!(
        r###"
$ echo 'This 42 is 1337 a t35t 666' | cx --sum
2045
    "###
    );
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
    return match log_path.try_exists()? {
        true => Ok(format!(
            "{} {}\n{}",
            "Log location:".italic().dimmed(),
            &log_path.display(),
            fs::read_to_string(&log_path)?
        )),
        false => Ok(format!(
            "{} {}",
            "No log file found:"
                .truecolor(250, 0, 104)
                .bold()
                .to_string(),
            log_path.display()
        )),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_words_test() {
        let input = "This is a test".to_string();
        let result = count_words(input);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_lines_test() {
        let input = "This\nis\na\ntest".to_string();
        let result = count_lines(input);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_chars_test() {
        let input = "This is a test".to_string();
        let result = count_chars(input);
        let expected = 11;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_bytes_test() {
        let input = "This is a test".to_string();
        let result = count_bytes(input);
        let expected = 14;
        assert_eq!(result, expected);
    }

    #[test]
    fn sum_test() {
        let input = "This 42 is 1337 a t35t 666".to_string();
        let result = sum(input);
        let expected = 2045;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_words_empty_test() {
        let input = "".to_string();
        let result = count_words(input);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_lines_empty_test() {
        let input = "".to_string();
        let result = count_lines(input);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_chars_empty_test() {
        let input = "".to_string();
        let result = count_chars(input);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_bytes_empty_test() {
        let input = "".to_string();
        let result = count_bytes(input);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn sum_empty_test() {
        let input = "".to_string();
        let result = sum(input);
        let expected = 0;
        assert_eq!(result, expected);
    }
}
