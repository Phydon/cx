[![Tests](https://github.com/Phydon/cx/actions/workflows/rust.yml/badge.svg)](https://github.com/Phydon/cx/actions/workflows/rust.yml)

# 🧮📄 cx

**C**ount **X**

* *count words*
* *count lines*
* *count chars*
* *count bytes*
* *count x*

## Examples

### Count lines, words, chars, bytes in file 

```shell
cat example.txt
# lorem ipsum wasd
```

```shell
cx example.txt
# 1 3 14 18 ./example.txt
```

#### Example: Count lines

```shell
cx example.txt --lines
# 1
```

#### Example: Count words

```shell
cx example.txt --words
# 3
```

#### Example: Count chars

```shell
cx example.txt --chars
# 14
```

#### Example: Count bytes

```shell
cx example.txt --bytes
# 18
```

### Count words in pipe 

```shell
echo 'Something interesting' | cx
# 1 2 20 23
```


## Usage

### Short Usage

```
Usage: cx [OPTIONS] [PATH] [COMMAND]

Commands:
  examples, --examples  Show examples
  log, -L, --log        Show content of the log file
  help                  Print this message or the help of the given subcommand(s)

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
  examples, --examples  Show examples
  log, -L, --log        Show content of the log file
  help                  Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]
          The filepath to work with
          Reads stdin if left empty

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
