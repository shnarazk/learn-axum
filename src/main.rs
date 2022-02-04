mod logging;
mod sample_middle;

use {
    axum::{
        extract::{Json, Path, Query},
        routing::get,
        Router,
    },
    logging::LogService,
    sample_middle::MyMiddleware,
    serde_json::json,
    std::collections::HashMap,
    tower::layer::layer_fn,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(get_foo_bar))
        .route("/json", get(json))
        .route("/path/:id", get(path))
        .route("/query", get(query))
        .route("/query_json", get(query_json))
        .route(
            "/m",
            get(|| async {
                dbg!();
            }),
        )
        .layer(layer_fn(|service| LogService::new(service, "test")))
        .layer(layer_fn(|inner| MyMiddleware { inner }));

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

async fn json() -> Json<serde_json::Value> {
    Json(json!({ "data": 42 }))
}

async fn path(Path(user_id): Path<u32>) -> &'static str {
    dbg!(user_id);
    "ok"
}

async fn query(Query(params): Query<HashMap<String, String>>) -> &'static str {
    dbg!(params);
    "this is query\n"
}

async fn query_json(Json(_payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(json!({ "data": 42 }))
}
