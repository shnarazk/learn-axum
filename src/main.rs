use {
    axum::{
        // body::Body,
        extract::{Json, Path, Query},
        routing::get,
        // response::Json,
        Router,
    },
    serde_json::json,
    std::collections::HashMap,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(get_foo_bar))
        .route("/path", get(path))
        .route("/query", get(query))
        .route("/query_json", get(json));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World from root!"
}

async fn get_foo() -> &'static str {
    "Hello, World from GET foo!"
}

async fn post_foo() -> &'static str {
    "Hello, World from POST foo!"
}

async fn get_foo_bar() -> &'static str {
    "Hello, World from GET foo/bar!"
}

async fn path(Path(_user_id): Path<u32>) {}

async fn query(Query(_params): Query<HashMap<String, String>>) {}

async fn json(Json(_payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(json!({ "data": 42 }))
}
