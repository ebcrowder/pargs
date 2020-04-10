/*!
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

 ```
 use std::env;

 fn main() {
    let actual_args: Vec<String> = env::args().collect();
    let command_args = vec![String::from("cool_command")];
    let flag_args = vec![String::from("-h")];
    let option_args = vec![String::from("-j"), String::from("-i")];

    let parsed_args = pargs::parse(actual_args, command_args, flag_args, option_args);

    match parsed_args {
        Ok(parsed_args) => println!("{:?}", parsed_args),
        Err(parsed_args) => println!("{}", parsed_args),
    }
 }
 ```

If we run this program with `cargo run cool_command -h -j=test123 -i=test456`,
 the output will be `{"flag_args": ["-h"], "command_args": ["cool_command"], "option_args": ["-j", "test123", "-i", "test456"]}`.

From here, we can lookup the values and utilize them in our program.
*/
use std::io::{Error, ErrorKind};
#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
/// maintains args that are successfully parsed and returns them to the calling program
pub struct Matches {
    pub command_args: Vec<String>,
    pub flag_args: Vec<String>,
    pub option_args: Vec<(String, String)>,
}

/// parses arguments in relation to expected optional and required arguments
pub fn parse(
    actual_args: Vec<String>,
    command_args: Vec<String>,
    flag_args: Vec<String>,
    option_args: Vec<String>,
) -> Result<Matches, Error> {
    let mut matches = Matches {
        command_args: Vec::new(),
        flag_args: Vec::new(),
        option_args: Vec::new(),
    };

    // return Error if no required arguments are provided
    if actual_args.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "no arguments were provided.",
        ));
    }

    // make a clone of actual_args vec for index lookup
    let actual_args_ref = actual_args.clone();

    // iterate over actual_args and match them with each arg category
    for (i, arg) in actual_args.iter().enumerate() {
        // parse option_args that use `=` into key value pairs
        let str_vec: Vec<&str> = match arg.contains("=") {
            true => arg.split("=").collect(),
            false => vec![&arg],
        };

        let split_key: String = match str_vec.len() {
            1 => "".to_string(),
            _ => str_vec[0].to_string(),
        };

        let split_value: String = match str_vec.len() {
            1 => "".to_string(),
            _ => str_vec[1].to_string(),
        };

        // insert args into relative Matches key
        if command_args.contains(&arg) {
            matches.command_args.push(arg.to_string())
        } else if flag_args.contains(&arg) {
            matches.flag_args.push(arg.to_string())
        } else if option_args.contains(&split_key) {
            matches.option_args.push((split_key, split_value));
        } else if option_args.contains(&arg) {
            // parse option args that use a ` ` into key value pairs
            matches
                .option_args
                .push((arg.to_string(), actual_args_ref[i + 1].to_string()));
        }
    }

    Ok(matches)
}
