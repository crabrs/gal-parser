#[macro_use]
mod common;

use gql_parser::Rule;

#[test]
fn return_item() {
    parse_assert!(Rule::return_item, "local_datetime({epochMillis: foaf.birthday}) AS birthday");
}