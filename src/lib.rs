// pargs
// command line argument parser
use std::io::{Error, ErrorKind};

#[cfg(test)]
mod tests;

/// parses arguments in relation to expected optional and required arguments
pub fn parse(
    actual_args: Vec<&str>,
    required_args: Vec<&str>,
    optional_args: Vec<&str>,
) -> Result<Vec<String>, Error> {
    let mut matches: Vec<String> = Vec::new();

    // return Error if no required arguments are provided
    if required_args.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "please provide at least one required argument",
        ));
    }

    // check for required args and return error if not matches
    for arg in &required_args {
        if !actual_args.contains(&arg) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "a required argument was not found",
            ));
        }
    }

    // iterate over actual arguments
    for arg in actual_args {
        if required_args.contains(&arg) || optional_args.contains(&arg) {
            matches.push(arg.to_string());
        }
    }

    Ok(matches)
}
