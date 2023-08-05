#[macro_use]
mod common;

use gql_parser::Rule;

#[test]
fn regular_identifier() {
    parse_assert!(Rule::regular_identifier, "n");
    parse_assert!(Rule::regular_identifier, "match_");
    parse_assert!(Rule::regular_identifier, "create_");
}

#[test]
fn reserved_word_identifier_locate() {
    parse_assert!(Rule::reserved_word_identifier_locate, "MATCH");
    parse_assert!(Rule::reserved_word_identifier_locate, "RETURN");
}

#[test]
#[should_panic]
fn invalid_reserved_word_identifier_locate() {
    parse_assert!(Rule::reserved_word_identifier_locate, "CREATE_");
}

#[test]
fn property_name() {
    parse_assert!(Rule::property_name, "createDate");
}

#[test]
#[should_panic]
fn reserved_word_except_day() {
    parse_assert!(Rule::reserved_word, "DAY");
}

#[test]
#[should_panic]
fn reserved_word_except_week() {
    parse_assert!(Rule::reserved_word, "WEEK");
}

#[test]
#[should_panic]
fn reserved_word_except_month() {
    parse_assert!(Rule::reserved_word, "MONTH");
}

#[test]
#[should_panic]
fn reserved_word_except_year() {
    parse_assert!(Rule::reserved_word, "YEAR");
}
