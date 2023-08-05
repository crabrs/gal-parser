#[macro_use]
mod common;

use gql_parser::Rule;


#[test]
fn identifer() {
    parse_assert!(Rule::identifier, "identifier");
    parse_assert!(Rule::identifier, "IDENTIFIER");
    parse_assert!(Rule::identifier, "IdEnTiFiEr");
    parse_assert!(Rule::identifier, "identifier123");
    parse_assert!(Rule::identifier, "identifier_12");
    parse_assert!(Rule::identifier, r#"@"identifier""#);
    parse_assert!(Rule::identifier, r#""identifier""#);
    parse_assert!(Rule::identifier, r#"@`identifier`"#);
    parse_assert!(Rule::identifier, r#"`identifier`"#);
}

#[test]
fn regular_identifier() {
    parse_assert!(Rule::regular_identifier, "identifier");
    parse_assert!(Rule::regular_identifier, "IDENTIFIER");
    parse_assert!(Rule::regular_identifier, "IdEnTiFiEr");
    parse_assert!(Rule::regular_identifier, "identifier123");
    parse_assert!(Rule::regular_identifier, "identifier_12");
}


#[test]
fn delimited_identifier() {
    parse_assert!(Rule::delimited_identifier, r#"@"identifier""#);
    parse_assert!(Rule::delimited_identifier, r#""identifier""#);
    parse_assert!(Rule::delimited_identifier, r#"@`identifier`"#);
    parse_assert!(Rule::delimited_identifier, r#"`identifier`"#);
}

#[test]
fn multiset_alternation_operator() {
    parse_assert!(Rule::multiset_alternation_operator, "|+|");
}


#[test]
fn bracket_right_arrow() {
    parse_assert!(Rule::bracket_right_arrow, "]->");
}

#[test]
fn bracket_tilde_right_arrow() {
    parse_assert!(Rule::bracket_tilde_right_arrow, "]~>");
}

#[test]
fn concatenation_operator() {
    parse_assert!(Rule::concatenation_operator, "||");
}

#[test]
fn double_colon() {
    parse_assert!(Rule::double_colon, "::");
}

#[test]
fn double_minus_sign() {
    parse_assert!(Rule::double_minus_sign, "--");
}

#[test]
fn double_period() {
    parse_assert!(Rule::double_period, "..");
}

#[test]
fn greater_than_operator() {
    parse_assert!(Rule::greater_than_operator, ">");
}

#[test]
fn greater_than_or_equals_operator() {
    parse_assert!(Rule::greater_than_or_equals_operator, ">=");
}

#[test]
fn left_arrow() {
    parse_assert!(Rule::left_arrow, "<-");
}

#[test]
fn left_arrow_tilde() {
    parse_assert!(Rule::left_arrow_tilde, "<~");
}

#[test]
fn left_arrow_bracket() {
    parse_assert!(Rule::left_arrow_bracket, "<-[");
}

#[test]
fn left_arrow_tilde_bracket() {
    parse_assert!(Rule::left_arrow_tilde_bracket, "<~[");
}

#[test]
fn left_minus_right() {
    parse_assert!(Rule::left_minus_right, "<->");
}

#[test]
fn left_minus_slash() {
    parse_assert!(Rule::left_minus_slash, "<-/");
}

#[test]
fn left_tilde_slash() {
    parse_assert!(Rule::left_tilde_slash, "<~/");
}

#[test]
fn less_than_operator() {
    parse_assert!(Rule::less_than_operator, "<");
}

#[test]
fn less_than_or_equals_operator() {
    parse_assert!(Rule::less_than_or_equals_operator, "<=");
}

#[test]
fn minus_left_bracket() {
    parse_assert!(Rule::minus_left_bracket, "-[");
}

#[test]
fn minus_slash() {
    parse_assert!(Rule::minus_slash, "-/");
}

#[test]
fn not_equals_operator() {
    parse_assert!(Rule::not_equals_operator, "<>");
}

#[test]
fn right_arrow() {
    parse_assert!(Rule::right_arrow, "->");
}

#[test]
fn right_bracket_minus() {
    parse_assert!(Rule::right_bracket_minus, "]-");
}

#[test]
fn right_bracket_tilde() {
    parse_assert!(Rule::right_bracket_tilde, "]~");
}

#[test]
fn slash_minus() {
    parse_assert!(Rule::slash_minus, "/-");
}

#[test]
fn slash_minus_right() {
    parse_assert!(Rule::slash_minus_right, "/->");
}

#[test]
fn slash_tilde() {
    parse_assert!(Rule::slash_tilde, "/~");
}

#[test]
fn slash_tilde_right() {
    parse_assert!(Rule::slash_tilde_right, "/~>");
}

#[test]
fn tilde_left_bracket() {
    parse_assert!(Rule::tilde_left_bracket, "~[");
}

#[test]
fn tilde_right_arrow() {
    parse_assert!(Rule::tilde_right_arrow, "~>");
}

#[test]
fn tilde_slash() {
    parse_assert!(Rule::tilde_slash, "~/");
}

#[test]
fn double_solidus() {
    parse_assert!(Rule::double_solidus, "//");
}

#[test]
fn separator() {
    parse_assert!(Rule::separator, " ");
    parse_assert!(Rule::separator, "\t");
    parse_assert!(Rule::separator, "\r");
    parse_assert!(Rule::separator, "\n");
    parse_assert!(Rule::separator, "// This is a comment\n");
    parse_assert!(Rule::separator, "/* This is a comment */");
}

#[test]
fn comment() {
    parse_assert!(Rule::comment, "// This is a comment\n");
    parse_assert!(Rule::comment, "/* This is a comment */");
    parse_assert!(
        Rule::comment,
        r#"/*
            This is multiple lines comments.
            First line...
            Second line...
        */"#
    );
}

#[test]
fn simple_comment() {
    parse_assert!(Rule::simple_comment, "// This is a comment\n");
}

#[test]
fn simple_comment_introducer() {
    parse_assert!(Rule::simple_comment_introducer, "//");
}

#[test]
fn bracketed_comment() {
    parse_assert!(Rule::bracketed_comment, "/* This is a comment */");
    parse_assert!(
        Rule::bracketed_comment,
        r#"/*
            This is multiple lines comments.
            First line...
            Second line...
        */"#
    );
}

#[test]
fn bracketed_comment_introducer() {
    parse_assert!(Rule::bracketed_comment_introducer, "/*");
}

#[test]
fn bracketed_comment_terminator() {
    parse_assert!(Rule::bracketed_comment_terminator, "*/");
}


#[test]
fn newline() {
    parse_assert!(Rule::newline, "\n");
    parse_assert!(Rule::newline, "\r");
    parse_assert!(Rule::newline, "\r\n");
}

#[test]
fn edge_synonym() {
    parse_assert!(Rule::edge_synonym, "edge");
    parse_assert!(Rule::edge_synonym, "EDGE");
    parse_assert!(Rule::edge_synonym, "EdGe");
    parse_assert!(Rule::edge_synonym, "relationship");
    parse_assert!(Rule::edge_synonym, "RELATIONSHIP");
    parse_assert!(Rule::edge_synonym, "ReLaTiOnShIp");
}

#[test]
fn edges_synonym() {
    parse_assert!(Rule::edges_synonym, "edges");
    parse_assert!(Rule::edges_synonym, "EDGES");
    parse_assert!(Rule::edges_synonym, "EdGeS");
    parse_assert!(Rule::edges_synonym, "relationships");
    parse_assert!(Rule::edges_synonym, "RELATIONSHIPS");
    parse_assert!(Rule::edges_synonym, "ReLaTiOnShIpS");
}

#[test]
fn node_synonym() {
    parse_assert!(Rule::node_synonym, "node");
    parse_assert!(Rule::node_synonym, "NODE");
    parse_assert!(Rule::node_synonym, "NoDe");
    parse_assert!(Rule::node_synonym, "VERTEX");
    parse_assert!(Rule::node_synonym, "vertex");
    parse_assert!(Rule::node_synonym, "vErTex");
}
