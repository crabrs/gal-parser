#[macro_use]
mod common;

use common::QueryResource;
use gql_parser::{GQLParser, Rule};
use pest::Parser;

#[test]
fn finbench() {
    test_queries!("finbench");
}

#[test]
fn snb() {
    test_queries!("snb");
}
