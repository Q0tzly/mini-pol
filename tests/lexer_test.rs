#[test]
#[ignore]
fn test_split() {
    let code = "str := Hello\nWorld";
    //let result: Vec<_> = code.split_whitespace().collect();
    let result: Vec<_> = code.split(" ").collect();
    assert_eq!(result, vec!["str", ":=", "Hello\nWorld"]);
}

#[test]
#[ignore]
fn test_is_keyword() {
    let code = "main";
    let keyword_list: [&str; 6] = ["main", "if", "el", "int", "str", "bol"];

    //let result = keyword_list.iter().any(|&keyword| keyword == code);
    let result = keyword_list.contains(&code);
    assert_eq!(result, true);
}

#[test]
#[ignore]
fn test_is_literal() {
    let code = "20";
    let is_integer = code.chars().all(|c| c.is_ascii_digit() || c == '-');
    assert_eq!(is_integer, true);

    let code = "\"Hello\n\"";
    let is_string = code.starts_with('"')
        && code.ends_with('"')
        && code.chars().filter(|&c| c == '"').count() == 2;
    assert_eq!(is_string, true);

    let code = "true";
    let is_bool = code == "true" || code == "false";
    assert_eq!(is_bool, true)
}

#[test]
#[ignore]
fn test_is_identifier() {
    let code = "fn_1";

    let is_empty = code.is_empty();
    assert_eq!(is_empty, false);

    let is_start_alphabet = code.chars().next().unwrap().is_ascii_lowercase();
    assert_eq!(is_start_alphabet, true);

    let is_right_str = code
        .chars()
        .skip(1)
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
    assert_eq!(is_right_str, true);
}

#[test]
#[ignore]
fn is_operator() {
    let code = ":=";
    let operator_list = ["+", "-", "*", "/", "%", ":=", "=", "!", "&", "<", ">"];
    let result;

    result = operator_list.iter().any(|&c| c == code);

    assert_eq!(result, true);
}
