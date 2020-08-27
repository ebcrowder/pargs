![](https://github.com/ebcrowder/pargs/workflows/pargs/badge.svg)
[![Latest version](https://img.shields.io/crates/v/pargs.svg)](https://crates.io/crates/pargs)
[![Documentation](https://docs.rs/pargs/badge.svg)](https://docs.rs/pargs)

# pargs

command line argument parser

pargs - command line argument parser

the design goal of pargs is to simply return parsed arguments
to a caller in a defined format for ease of lookup.

pargs works with three common types of arguments:
commands, flags and options.

# Using pargs

using pargs is very simple:
define all three types of arguments that your program needs
and pass them as individual `Vec<String>` to `pargs::parse()`.
`parse()` will return a `Matches` struct of the parsed arguments
keyed by category so that your application can easily
interpret them.

The return values of successfully parsed arguments are as follows:

- command_args: `Vec<String>`
- flag_args: `Vec<String>`
- option_args: `HashMap`

# Definitions

- `command_args` are defined as single arguments that do not have an assigned value
- `command_args` args should be entered without a dash
- `flag_args` are intended to be boolean values
- `flag_args` should not be assigned a value - if they exist, they are interpreted as `true`
- `option_args` should be assigned a value
- `option_args` should be denoted with a `-` character
- `option_args` can be assigned a value via `=` or space between arg and value

# Example

The following example shows a simple program that defines all three types of arguments
(commands, flag and option). Pargs is passed a `Vec<String>` from `env::args()`
at which point it parses the arguments and returns them to the program in a simple data structure.

```rust
use std::env;
use pargs::Pargs;

let actual_args: Vec<String> = env::args().collect();
let command_args = vec![String::from("cool_command")];
let flag_args = vec![String::from("-h")];
let option_args = vec![String::from("-j"), String::from("-i")];

let parsed_args = Pargs::parse(actual_args, command_args, flag_args, option_args);

match parsed_args {
    Ok(parsed_args) => println!("{:?}", parsed_args),
    Err(parsed_args) => println!("{}", parsed_args),
}
```

If we run this program with `cargo run cool_command -h -j=test123 -i=test456`,
the output will be `Matches { command_args: ["cool_command"], flag_args: ["-h"], option_args: {"-i": "test456", "-j": "test123"} }`.

From here, we can lookup the values and utilize them in our program.
