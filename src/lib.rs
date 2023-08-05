use pest::{iterators::Pairs, Parser, error::Error};

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "gql.pest"] // relative to src
pub struct GQLParser;

pub fn parse(gql: &str) -> Result<Pairs<'_, Rule>, Error<Rule>> {
    GQLParser::parse(Rule::gql_request, gql)
}