use super::parse;

#[test]
fn parse_returns_result() {
    let options = vec!["filename", "-h", "-o"];

    let actual = parse(options);

    match actual {
        Ok(actual) => assert_eq!(actual.opts.len(), 3),
        _ => panic!(),
    }
}
