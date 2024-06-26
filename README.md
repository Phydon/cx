[![Tests](https://github.com/Phydon/cx/actions/workflows/rust.yml/badge.svg)](https://github.com/Phydon/cx/actions/workflows/rust.yml)

# 🧮 📄 cx

**C**ount **X**

* *count words*
* *count lines*
* *count chars*
* *count bytes*
* *count x*

## Examples

Count words, lines, chars, bytes in file 

![screenshot](https://github.com/Phydon/cx/blob/master/assets/cx_file.png)

Count words in pipe 

![screenshot](https://github.com/Phydon/cx/blob/master/assets/cx_pipe.png)

## Usage

### Short Usage

```
Usage: cx [OPTIONS] [PATH] [COMMAND]

Commands:
  log, -L, --log  Show content of the log file
  help            Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]  The filepath to work with

Options:
  -b, --bytes        Count all bytes [aliases: byte]
  -c, --chars        Count all chars [aliases: char]
  -l, --lines        Count all lines [aliases: line]
  -S, --show-errors  Show errors (ignores errors by default) [aliases: show-error]
  -w, --words        Count all words [aliases: word]
  -h, --help         Print help (see more with '--help')
  -V, --version      Print version
```

### Long Usage

```
Usage: cx [OPTIONS] [PATH] [COMMAND]

Commands:
  log, -L, --log  Show content of the log file
  help            Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]
          The filepath to work with

Options:
  -b, --bytes
          Count all bytes

          [aliases: byte]

  -c, --chars
          Count all chars

          [aliases: char]

  -l, --lines
          Count all lines

          [aliases: line]

  -S, --show-errors
          Show errors (ignores errors by default)

          [aliases: show-error]

  -w, --words
          Count all words

          [aliases: word]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```


## Installation

### Windows

via Cargo or get the ![binary](https://github.com/Phydon/cx/releases)
