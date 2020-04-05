use super::parse;
use std::collections::HashMap;

#[test]
fn parse_returns_result() {
    let args = vec![
        String::from("pizza_command"),
        String::from("-h"),
        String::from("-i=peppers"),
        String::from("-j=mushrooms"),
    ];
    let command_args = vec![String::from("pizza_command")];
    let flag_args = vec![String::from("-h")];
    let option_args = vec![String::from("-i=peppers"), String::from("-j=mushrooms")];

    let actual = parse(args, command_args, flag_args, option_args);

    let mut expected: HashMap<String, Vec<String>> = HashMap::new();

    expected.insert(
        "command_args".to_string(),
        vec!["pizza_command".to_string()],
    );
    expected.insert("flag_args".to_string(), vec!["-h".to_string()]);
    expected.insert(
        "option_args".to_string(),
        vec![
            "-i".to_string(),
            "peppers".to_string(),
            "-j".to_string(),
            "mushrooms".to_string(),
        ],
    );

    match actual {
        Ok(actual) => assert_eq!(actual, expected),
        _ => panic!(),
    }
}

#[test]
fn parse_returns_error_if_reqd_args_not_defined() {
    let args = vec![];
    let command_args = vec![String::from("filename")];
    let flag_args = vec![String::from("-h")];
    let option_args = vec![String::from("-j=hi"), String::from("-i=there")];

    let actual = parse(args, command_args, flag_args, option_args);

    match actual {
        Err(_actual) => {}
        _ => panic!(),
    }
}
