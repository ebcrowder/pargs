// pargs
// command line argument parser
use std::io::{Error, ErrorKind};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Options<T> {
    opts: Vec<T>,
}

pub fn parse<T>(options: Vec<T>) -> Result<Options<T>, Error> {
    let mut results = Options { opts: Vec::new() };

    for option in options {
        results.opts.push(option);
    }

    return Ok(results);
}
