use super::parse;
use super::Matches;

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

    let actual = parse(args, command_args, flag_args, option_args);

    let mut expected = Matches {
        command_args: Vec::new(),
        flag_args: Vec::new(),
        option_args: Vec::new(),
    };

    expected.command_args.push("pizza_command".to_string());
    expected.flag_args.push("-h".to_string());
    expected
        .option_args
        .push(("-i".to_string(), "peppers".to_string()));

    expected
        .option_args
        .push(("-j".to_string(), "mushrooms".to_string()));
    expected
        .option_args
        .push(("-z".to_string(), "cheez".to_string()));

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

    let actual = parse(args, command_args, flag_args, option_args);

    match actual {
        Err(_actual) => {}
        _ => panic!(),
    }
}
