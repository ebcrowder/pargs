// pargs
// command line argument parser
use std::io::{Error, ErrorKind};

#[cfg(test)]
mod tests;

/// parses arguments in relation to expected optional and required arguments
pub fn parse(
    actual_args: Vec<String>,
    expected_required_args: Vec<String>,
    expected_optional_args: Vec<String>,
) -> Result<Vec<String>, Error> {
    let mut matches: Vec<String> = Vec::new();

    // return Error if no required arguments are provided
    if expected_required_args.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "please provide at least one required argument",
        ));
    }

    // check for required args and return error if not matches
    for arg in &expected_required_args {
        if !actual_args.contains(&arg) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "a required argument was not found",
            ));
        }
    }

    // iterate over actual arguments
    for arg in actual_args {
        if expected_required_args.contains(&arg) || expected_optional_args.contains(&arg) {
            matches.push(arg);
        }
    }

    Ok(matches)
}
