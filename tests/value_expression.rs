#[macro_use]
mod common;

use gql_parser::Rule;

#[test]
fn exact_numeric_literal() {
    parse_assert!(Rule::exact_numeric_literal, "30.0");
}

#[test]
fn result_expression() {
    parse_assert!(Rule::result_expression, "FLOOR(n / a * b) / 1000");
}

#[test]
fn case_expression() {
    parse_assert!(Rule::case_expression, "CASE n WHEN IS NULL THEN 1 ELSE 0 END");
    parse_assert!(Rule::case_expression, "CASE WHEN a > b THEN 1 ELSE 0 END");
    parse_assert!(Rule::case_expression, "CASE p WHEN IS NULL THEN -1 ELSE path_length(p) END");
}

#[test]
fn when_operand() {
    parse_assert!(Rule::when_operand, "IS NULL");
}


#[test]
fn path_length_expression() {
    parse_assert!(Rule::path_length_expression, "path_length(p)");
}

#[test]
fn common_value_expression() {
    parse_assert!(Rule::common_value_expression, "DURATION_BETWEEN(n.creationDate, $creationDate)");
}

#[test]
fn duration_value_expression() {
    parse_assert!(Rule::duration_value_expression, "DURATION_BETWEEN(n.creationDate, $creationDate)");
}

#[test]
fn boolean_value_expression() {
    parse_assert!(Rule::boolean_value_expression, "DURATION_BETWEEN(n.creationDate, $creationDate) > 10");
}

#[test]
fn datetime_value_expression() {
    parse_assert!(Rule::local_datetime_function, "local_datetime({epochMillis: foaf.birthday})");
}

#[test]
fn list_value_constructor() {
    parse_assert!(Rule::list_value_constructor, "[n.field, 2, m]");
}

#[test]
fn property_reference() {
    parse_assert!(Rule::property_reference, "n.field");
    parse_assert!(Rule::property_reference, "n.field.subfield");
    parse_assert!(Rule::property_reference, "n.field.subfield.subsubfield");
}