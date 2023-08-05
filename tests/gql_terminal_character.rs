#[macro_use]
mod common;

use gql_parser::Rule;

#[test]
fn hex_digit() {
    for digit in ('0'..='9')
        .chain('a'..='f')
        .chain('A'..='F')
        .map(|c| c.to_string())
    {
        parse_assert!(Rule::hex_digit, &digit);
    }
}

#[test]
fn digit() {
    for digit in ('0'..='9').map(|c| c.to_string()) {
        parse_assert!(Rule::digit, &digit);
    }
}

#[test]
fn standard_digit() {
    for digit in ('0'..='9').map(|c| c.to_string()) {
        parse_assert!(Rule::standard_digit, &digit);
    }
}

#[test]
fn octal_digit() {
    for digit in ('0'..='7').map(|c| c.to_string()) {
        parse_assert!(Rule::octal_digit, &digit);
    }
}

#[test]
fn binary_digit() {
    for digit in ('0'..='1').map(|c| c.to_string()) {
        parse_assert!(Rule::binary_digit, &digit);
    }
}

#[test]
#[ignore = "not supported"]
fn other_digit() {}


#[test]
fn space() {
    parse_assert!(Rule::space, " ");
}

#[test]
fn ampersand() {
    parse_assert!(Rule::ampersand, "&");
}

#[test]
fn asterisk() {
    parse_assert!(Rule::asterisk, "*");
}

#[test]
fn colon() {
    parse_assert!(Rule::colon, ":");
}

#[test]
fn comma() {
    parse_assert!(Rule::comma, ",");
}

#[test]
fn commercial_at() {
    parse_assert!(Rule::commercial_at, "@");
}

#[test]
fn dollar_sign() {
    parse_assert!(Rule::dollar_sign, "$");
}

#[test]
fn double_quote() {
    parse_assert!(Rule::double_quote, "\"");
}

#[test]
fn equals_operator() {
    parse_assert!(Rule::equals_operator, "=");
}

#[test]
fn exclamation_mark() {
    parse_assert!(Rule::exclamation_mark, "!");
}

#[test]
fn grave_accent() {
    parse_assert!(Rule::grave_accent, "`");
}

#[test]
fn right_angle_bracket() {
    parse_assert!(Rule::right_angle_bracket, ">");
}

#[test]
fn left_brace() {
    parse_assert!(Rule::left_brace, "{");
}

#[test]
fn left_bracket() {
    parse_assert!(Rule::left_bracket, "[");
}

#[test]
fn left_paren() {
    parse_assert!(Rule::left_paren, "(");
}

#[test]
fn left_angle_bracket() {
    parse_assert!(Rule::left_angle_bracket, "<");
}

#[test]
fn minus_sign() {
    parse_assert!(Rule::minus_sign, "-");
}

#[test]
fn percent() {
    parse_assert!(Rule::percent, "%");
}

#[test]
fn period() {
    parse_assert!(Rule::period, ".");
}

#[test]
fn plus_sign() {
    parse_assert!(Rule::plus_sign, "+");
}

#[test]
fn question_mark() {
    parse_assert!(Rule::question_mark, "?");
}

#[test]
fn quote() {
    parse_assert!(Rule::quote, "'");
}

#[test]
fn reverse_solidus() {
    parse_assert!(Rule::reverse_solidus, "\\");
}

#[test]
fn right_brace() {
    parse_assert!(Rule::right_brace, "}");
}

#[test]
fn right_bracket() {
    parse_assert!(Rule::right_bracket, "]");
}

#[test]
fn right_paren() {
    parse_assert!(Rule::right_paren, ")");
}

#[test]
fn semicolon() {
    parse_assert!(Rule::semicolon, ";");
}

#[test]
fn solidus() {
    parse_assert!(Rule::solidus, "/");
}

#[test]
fn tilde() {
    parse_assert!(Rule::tilde, "~");
}

#[test]
fn underscore() {
    parse_assert!(Rule::underscore, "_");
}

#[test]
fn vertical_bar() {
    parse_assert!(Rule::vertical_bar, "|");
}

#[test]
#[ignore = "not supported"]
fn other_language_character() {}
