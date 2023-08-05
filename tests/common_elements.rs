#[macro_use]
mod common;

use gql_parser::Rule;

#[test]
fn edge_pattern() {
    parse_assert!(Rule::edge_pattern, "<-[r:label]->");
    parse_assert!(Rule::edge_pattern, "-[r]->");
    parse_assert!(Rule::edge_pattern, "<-[r]-");
    parse_assert!(Rule::edge_pattern, "~[r]~>");
    parse_assert!(Rule::edge_pattern, "<~[r:label]~");
    parse_assert!(Rule::edge_pattern, "~[r]~");
    parse_assert!(Rule::edge_pattern, "-[r:label]-");
}

#[test]
fn path_factor() {
    parse_assert!(Rule::path_factor, "<-[r:label]->*");
    parse_assert!(Rule::path_factor, "<-[r:label]->+");
    parse_assert!(Rule::path_factor, "<-[r:label]->{2}");
    parse_assert!(Rule::path_factor, "<-[r:label]->{1,}");
    parse_assert!(Rule::path_factor, "<-[r:label]->{1,2}");
    parse_assert!(Rule::path_factor, "<-[r:label]->{,2}");    
}

#[test]
fn node_pattern() {
    for input in [
        "(n:Label WHERE DURATION_BETWEEN(n.createDate, $startDate) >= 0)",
    ] {
        parse_assert!(Rule::node_pattern, input);
    }
}

#[test]
fn element_pattern_where_clause() {
    for input in [
        "WHERE DURATION_BETWEEN(n.createDate, $startDate) >= 0",
    ] {
        parse_assert!(Rule::element_pattern_where_clause, input);
    }
}

#[test]
fn insert_element_pattern_filter() {
    parse_assert!(Rule::insert_element_pattern_filter, "n Label");
    parse_assert!(Rule::insert_element_pattern_filter, "n Label {field: $value}");
    parse_assert!(Rule::insert_element_pattern_filter, "Label {field: $value}");
    parse_assert!(Rule::insert_element_pattern_filter, "Label");
}