use std::{time::Instant, hint::black_box};
use gql_parser::{GQLParser, Rule};
use pest::Parser;

fn main() {
    let start = Instant::now();
    let gql = include_str!("../queries/snb/complex_read_1.gql");
    let _ = GQLParser::parse(Rule::gql_request, gql).unwrap().for_each(|p| {
        black_box(p);
    });
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}