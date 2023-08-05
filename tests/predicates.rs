#[macro_use]
mod common;

use gql_parser::Rule;

#[test]
fn comparison_predicand() {
    parse_assert!(Rule::comparison_predicand, "DURATION_BETWEEN(n.creationDate, $creationDate)");
    parse_assert!(Rule::comparison_predicand, "30.0");
}


#[test]
fn predicate() {
    parse_assert!(Rule::predicate, "DURATION_BETWEEN(n.creationDate, $creationDate) > 10");
}

#[test]
fn search_condition() {
    parse_assert!(Rule::search_condition, "DURATION_BETWEEN(n.createDate, $startDate) > 0");
}