use super::parse;

#[test]
fn parse_returns_result() {
    let args = vec!["filename", "-h", "-o"];
    let required_args = vec!["filename"];
    let optional_args = vec!["-h", "-o"];

    let actual = parse(args, required_args, optional_args);

    let expected = vec!["filename", "-h", "-o"];

    match actual {
        Ok(actual) => assert_eq!(actual, expected),
        _ => panic!(),
    }
}

#[test]
fn parse_returns_result_lots_of_args() {
    let args = vec!["filename", "-p", "-a", "-r", "-g", "-s"];

    let required_args = vec!["filename"];

    let optional_args = vec!["-p", "-a", "-r", "-g", "-s"];

    let actual = parse(args, required_args, optional_args);

    let expected = vec!["filename", "-p", "-a", "-r", "-g", "-s"];

    match actual {
        Ok(actual) => assert_eq!(actual, expected),
        _ => panic!(),
    }
}

#[test]
fn parse_only_returns_matching_args() {
    let args = vec!["filename", "-p", "-a", "-r", "-g", "-s", "-v", "-i", "-m"];

    let required_args = vec!["filename"];

    let optional_args = vec!["-p", "-a", "-r", "-g", "-s"];

    let actual = parse(args, required_args, optional_args);

    let expected = vec!["filename", "-p", "-a", "-r", "-g", "-s"];

    match actual {
        Ok(actual) => assert_eq!(actual, expected),
        _ => panic!(),
    }
}

#[test]
fn parse_returns_error_if_reqd_args_missing() {
    let args = vec!["-h", "-o"];
    let required_args = vec!["filename"];
    let optional_args = vec!["-h", "-o"];

    let actual = parse(args, required_args, optional_args);

    match actual {
        Err(_actual) => {}
        _ => panic!(),
    }
}

#[test]
fn parse_returns_error_if_reqd_args_not_defined() {
    let args = vec!["-h", "-o"];
    let required_args = vec![];
    let optional_args = vec!["-h", "-o"];

    let actual = parse(args, required_args, optional_args);

    match actual {
        Err(_actual) => {}
        _ => panic!(),
    }
}
