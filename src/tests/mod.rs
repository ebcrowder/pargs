use super::parse;
use std::collections::HashMap;

#[test]
fn parse_returns_result() {
    let args = vec![
        String::from("filename"),
        String::from("-h"),
        String::from("-o"),
    ];
    let required_args = vec![String::from("filename")];
    let optional_args = vec![String::from("-h"), String::from("-o")];

    let actual = parse(args, required_args, optional_args);

    let expected = vec![
        String::from("filename"),
        String::from("-h"),
        String::from("-o"),
    ];

    match actual {
        Ok(actual) => assert_eq!(actual, expected),
        _ => panic!(),
    }
}
