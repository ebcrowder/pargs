use super::Pargs;
use std::collections::HashMap;

#[test]
fn parse_returns_result() {
    let args = vec![
        String::from("pizza_command"),
        String::from("-h"),
        String::from("-i=peppers"),
        String::from("-j=mushrooms"),
        String::from("-z"),
        String::from("cheez"),
    ];
    let command_args = vec![String::from("pizza_command")];
    let flag_args = vec![String::from("-h")];
    let option_args = vec![String::from("-i"), String::from("-j"), String::from("-z")];

    let actual = Pargs::parse(args, command_args, flag_args, option_args);

    let mut expected = Pargs {
        command_args: Vec::new(),
        flag_args: Vec::new(),
        option_args: HashMap::new(),
    };

    expected.command_args.push(String::from("pizza_command"));
    expected.flag_args.push(String::from("-h"));
    expected
        .option_args
        .insert(String::from("-i"), String::from("peppers"));
    expected
        .option_args
        .insert(String::from("-j"), String::from("mushrooms"));
    expected
        .option_args
        .insert(String::from("-z"), String::from("cheez"));

    match actual {
        Ok(actual) => assert_eq!(actual, expected),
        _ => panic!(),
    }
}

#[test]
fn parse_returns_error_if_reqd_args_not_defined() {
    let args = vec![];
    let command_args = vec![String::from("pizza_command")];
    let flag_args = vec![String::from("-h")];
    let option_args = vec![String::from("-i"), String::from("-j")];

    let actual = Pargs::parse(args, command_args, flag_args, option_args);

    match actual {
        Err(_e) => {}
        _ => panic!(),
    }
}
