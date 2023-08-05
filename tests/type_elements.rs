#[macro_use]
mod common;

use gql_parser::Rule;

#[test]
fn signed_binary_exact_numeric_type() {
    for input in [
        "INT",
        "INT8",
        "INT16",
        "INT32",
        "INT64",
        "INT128",
        "INT256",
        "SMALLINT",
        "BIGINT",
        "SIGNED INTEGER",
        "INTEGER8",
        "INTEGER16",
        "INTEGER32",
        "INTEGER64",
        "INTEGER128",
        "INTEGER256",
    ] {
        parse_assert!(Rule::signed_binary_exact_numeric_type, input);
    }
}
