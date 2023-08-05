#[macro_use]
mod common;

use gql_parser::Rule;

#[test]
fn group_by_clause() {
    parse_assert!(Rule::group_by_clause, "GROUP BY NP_LONG");
}