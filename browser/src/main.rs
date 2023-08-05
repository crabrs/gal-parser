use std::{time::Instant, path::Path, fs, env};

use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use gql_parser::Rule;
use pest::iterators::Pair;
use serde_json::{json, Value};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(index_html),
        )
        .route(
            "/index.html",
            get(index_html),
        )
        .route(
            "/version",
            get(|| async {
                format!("Version: {VERSION}")
            })
        )
        .route("/parse", post(parse));


    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_html() -> Html<String> {
    resource("web/index.html").await
}

async fn resource<P: AsRef<Path>>(path: P) -> Html<String> {
    Html(fs::read_to_string(path).unwrap_or("resource not found".into()))
}

async fn parse(Json(payload): Json<serde_json::Value>) -> (StatusCode, Json<Value>) {
    let gql = payload.get("gql").and_then(|v| v.as_str()).unwrap();
    match parse_gql(gql) {
        Ok(v) => (StatusCode::OK, Json(v)),
        Err(err_msg) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                    "err_msg": err_msg
            })),
        ),
    }
}

fn parse_gql(gql: &str) -> Result<Value, String> {
    fn pair_to_json<'i>(pair: Pair<'i, Rule>) -> Value {
        let rule = format!("{:?}", pair.as_rule());
        let term = pair.as_str();
        let children: Vec<_> = pair.into_inner().map(|child| pair_to_json(child)).collect();
        if children.is_empty() {
            json!({
                "rule": rule,
                "term": term,
            })
        } else {
            json!({
                "rule": rule,
                "children": children,
            })
        }
    }
    let start = Instant::now();
    let mut pairs = gql_parser::parse(gql).map_err(|e| e.to_string())?;
    let elapsed = start.elapsed();

    pairs
        .next()
        .ok_or("empty result".into())
        .map(|root| {
            let parse_tree = pair_to_json(root);
            json!({
                "elapsed": format!("{:?}", elapsed),
                "parse_tree": parse_tree,
            })
        })
}
