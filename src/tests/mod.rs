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

    let mut expected: HashMap<String, Vec<String>> = HashMap::new();

    expected.insert("required_args".to_string(), vec!["filename".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-h".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-o".to_string()]);

    match actual {
        Ok(actual) => assert_eq!(actual, expected),
        _ => panic!(),
    }
}

#[test]
fn parse_returns_result_lots_of_args() {
    let args = vec![
        String::from("filename"),
        String::from("-p"),
        String::from("-a"),
        String::from("-r"),
        String::from("-g"),
        String::from("-s"),
    ];

    let required_args = vec![String::from("filename")];

    let optional_args = vec![
        String::from("-p"),
        String::from("-a"),
        String::from("-r"),
        String::from("-g"),
        String::from("-s"),
    ];

    let actual = parse(args, required_args, optional_args);

    let mut expected: HashMap<String, Vec<String>> = HashMap::new();

    expected.insert("required_args".to_string(), vec!["filename".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-p".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-a".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-r".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-g".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-s".to_string()]);

    match actual {
        Ok(actual) => assert_eq!(actual, expected),
        _ => panic!(),
    }
}

#[test]
fn parse_only_returns_matching_args() {
    let args = vec![
        String::from("filename"),
        String::from("-p"),
        String::from("-a"),
        String::from("-r"),
        String::from("-g"),
        String::from("-s"),
        String::from("-v"),
        String::from("-i"),
        String::from("-m"),
    ];

    let required_args = vec![String::from("filename")];

    let optional_args = vec![
        String::from("-p"),
        String::from("-a"),
        String::from("-r"),
        String::from("-g"),
        String::from("-s"),
    ];

    let actual = parse(args, required_args, optional_args);

    let mut expected: HashMap<String, Vec<String>> = HashMap::new();

    expected.insert("required_args".to_string(), vec!["filename".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-p".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-a".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-r".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-g".to_string()]);
    expected.insert("optional_args".to_string(), vec!["-s".to_string()]);

    match actual {
        Ok(actual) => assert_eq!(actual, expected),
        _ => panic!(),
    }
}

#[test]
fn parse_returns_error_if_reqd_args_missing() {
    let args = vec![String::from("-h"), String::from("-o")];
    let required_args = vec![String::from("filename")];
    let optional_args = vec![String::from("-h"), String::from("-o")];

    let actual = parse(args, required_args, optional_args);

    match actual {
        Err(_actual) => {}
        _ => panic!(),
    }
}

#[test]
fn parse_returns_error_if_reqd_args_not_defined() {
    let args = vec![String::from("-h"), String::from("-o")];
    let required_args = vec![];
    let optional_args = vec![String::from("-h"), String::from("-o")];

    let actual = parse(args, required_args, optional_args);

    match actual {
        Err(_actual) => {}
        _ => panic!(),
    }
}
