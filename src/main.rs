// TODO handle multiple files
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
        let mut filepath = PathBuf::new();

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
            filepath.push(path);
        } else {
            // read input from stdin
            let input = read_stdin();
            content.push_str(&input);
        }

        // INFO use usize over u64 (applies to code block & functions below)

        // usize is pointer-sized, thus its actual size depends on the architecture you are compiling your program for.
        // As an example, on a 32 bit x86 computer, usize = u32, while on x86_64 computers, usize = u64.
        // usize gives you the guarantee to be always big enough to hold any pointer or any offset in a data structure,
        // while u32 can be too small on some architectures

        // WARNING This answer is legacy for Rust, usize have been redefined as "can hold any memory location" ->
        // TL;DR: a pointer is not just a number
        // https://stackoverflow.com/questions/29592256/whats-the-difference-between-usize-and-u32

        let lines: usize = count_lines(&content);
        let words: usize = count_words(&content);
        let bytes: usize = count_bytes(&content);
        let chars: usize = count_chars(&content);

        if word_flag {
            println!("{}", words);
        } else if lines_flag {
            println!("{}", lines);
        } else if chars_flag {
            println!("{}", chars);
        } else if bytes_flag {
            println!("{}", bytes);
        } else {
            let out = if filepath.as_path().exists() {
                format!(
                    "{} {} {} {} {}",
                    lines,
                    words,
                    chars,
                    bytes,
                    filepath.as_path().display()
                )
            } else {
                format!("{} {} {} {}", lines, words, chars, bytes)
            };

            println!("{}", out);
        }
    }
}

fn count_words(content: &str) -> usize {
    content.par_split_whitespace().count()
}

fn count_lines(content: &str) -> usize {
    content.par_lines().count()
}

fn count_chars(content: &str) -> usize {
    // TODO process in parallel
    let mut count = 0;
    content.split_whitespace().for_each(|word| {
        word.chars().for_each(|_| {
            count += 1;
        })
    });

    count as usize

    // TODO FIXME
    // let mut count = 0;
    // content.par_split_whitespace().for_each(|word| {
    //     word.par_chars().for_each(|_| {
    //         count += 1;
    //     })
    // });
}

fn count_bytes(content: &str) -> usize {
    content.par_bytes().count()
}

// FIXME incorrect counting of bytes when content is read via stdin (works when reading from file)
fn read_stdin() -> String {
    let mut input = io::stdin()
        .lock()
        .lines()
        // Returns an iterator over the lines of this reader.
        // The iterator returned from this function will yield instances of io::Result<String>. Each string returned will not have a newline byte (the 0xA byte) or CRLF (0xD, 0xA bytes) at the end.
        // FIXME INFO this results in the loss of the 2 missing bytes -> append at each line at the end?? or find another way to handle stdin
        .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");

    // TODO possible error here?
    // TODO if last char is '\n' it will get removed
    // INFO this has nothing to do with the incorrect counting of bytes (most likely not)
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
        .long_about(format!(
            "{}\n{}\n{}",
            "Count X",
            "Count lines, words, chars, bytes.",
            "Read content from stdin or filepaths as argument.",
        ))
        // TODO update version
        .version("1.3.0")
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

$ cx example.txt
1 3 14 19 example.txt

$ cx example.txt --lines
1

$ cx example.txt --words
3

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
1 3 13 17
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
        let result = count_words(&input);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_lines_test() {
        let input = "This\nis\na\ntest".to_string();
        let result = count_lines(&input);
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_chars_test() {
        let input = "This is a test".to_string();
        let result = count_chars(&input);
        let expected = 11;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_bytes_test() {
        let input = "This is a test".to_string();
        let result = count_bytes(&input);
        let expected = 14;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_words_empty_test() {
        let input = "".to_string();
        let result = count_words(&input);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_lines_empty_test() {
        let input = "".to_string();
        let result = count_lines(&input);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_chars_empty_test() {
        let input = "".to_string();
        let result = count_chars(&input);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn count_bytes_empty_test() {
        let input = "".to_string();
        let result = count_bytes(&input);
        let expected = 0;
        assert_eq!(result, expected);
    }
}
