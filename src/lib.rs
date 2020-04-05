/// # pargs
/// ### command line argument parser

/// pargs works with three common types of arguments:
/// commands, flags and options

/// using pargs is very simple:
/// define all three types of arguments that your program needs
/// and pass them as individual `Vec<String>` to `pargs::parse();`
/// `parse()` will return a `HashMap` of the parsed arguments
///  keyed by category so that your application can easily
/// interpret them.
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

    // iterate over actual_args and match them with each arg category
    for arg in actual_args {
        // parse option_args into key value pairs
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
                .push(arg)
        } else if flag_args.contains(&arg) {
            matches
                .entry("flag_args".to_string())
                .or_insert(Vec::new())
                .push(arg)
        } else if option_args.contains(&split_key) {
            matches
                .entry("option_args".to_string())
                .or_insert(Vec::new())
                .push(split_key);

            matches
                .entry("option_args".to_string())
                .or_insert(Vec::new())
                .push(split_value);
        }
    }

    Ok(matches)
}
