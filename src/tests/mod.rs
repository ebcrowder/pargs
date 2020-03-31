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

    let hash_map_expected = HashMap::new();

    hash_map_expected.insert("filename", 1);
    hash_map_expected.insert("-h", 1);
    hash_map_expected.insert("-o", 1);

    match actual {
        Ok(actual) => assert_eq!(actual, hash_map_expected),
        _ => panic!(),
    }
}
