#[macro_use]
mod common;

use gql_parser::Rule;

#[test]
fn return_statement() {
    parse_assert!(Rule::return_statement, "RETURN /*+ read_consistency( none ) */ 1, n.p_long");
}