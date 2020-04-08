/*! # pargs
 ### command line argument parser

 pargs works with three common types of arguments:
 commands, flags and options

 # Using pargs
 using pargs is very simple:
 define all three types of arguments that your program needs
 and pass them as individual `Vec<String>` to `pargs::parse()`.
 `parse()` will return a `HashMap` of the parsed arguments
  keyed by category so that your application can easily
 interpret them.

 # Definitions
 `command` args are defined as single arguments that do not have an assigned value
 `command` args should be entered without a dash
 `flag` args are intended to be boolean values
 `flag` args should not be assigned a value - if they exist, they are interpreted as `true`
 `option` args should be assigned a value
 `option` args should be denoted with a `-` character
 `option` args can be assigned a value via `=` or space between arg and value

 # Example

 The following example shows a simple program that defines all three types of arguments
 (commands, flag and option). Pargs is passed a `Vec<String>` from `env::args()`
 at which point it parses the arguments and returns them to the program in a `HashMap<String,
 Vec<String>>` data structure.

 ```{.rust}
use pargs;
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

 if we run this program with `cargo run cool_command -h -j=test123 -i=test456`,
 the output will be `{"flag_args": ["-h"], "command_args": ["cool_command"], "option_args": ["-j", "test123", "-i", "test456"]}`
*/

use std::collections::HashMap;
use std::io::{Error, ErrorKind};

#[cfg(test)]
mod tests;
/// parses arguments in relation to expected optional and required arguments
pub fn parse(
    actual_args: Vec<String>,
    command_args: Vec<String>,
    flag_args: Vec<String>,
    option_args: Vec<String>,
) -> Result<HashMap<String, Vec<String>>, Error> {
    let mut matches: HashMap<String, Vec<String>> = HashMap::new();

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

        // insert args into relative `HashMap` key
        if command_args.contains(&arg) {
            matches
                .entry("command_args".to_string())
                .or_insert(Vec::new())
                .push(arg.to_string())
        } else if flag_args.contains(&arg) {
            matches
                .entry("flag_args".to_string())
                .or_insert(Vec::new())
                .push(arg.to_string())
        } else if option_args.contains(&split_key) {
            matches
                .entry("option_args".to_string())
                .or_insert(Vec::new())
                .push(split_key);

            matches
                .entry("option_args".to_string())
                .or_insert(Vec::new())
                .push(split_value);
        } else if option_args.contains(&arg) {
            // parse option args that use a ` ` into key value pairs
            matches
                .entry("option_args".to_string())
                .or_insert(Vec::new())
                .push(arg.to_string());
            matches
                .entry("option_args".to_string())
                .or_insert(Vec::new())
                .push(actual_args_ref[i + 1].to_string());
        }
    }

    Ok(matches)
}
