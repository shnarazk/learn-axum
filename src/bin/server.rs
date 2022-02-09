use {
    axum::{
        extract::{Json, Path, Query}, // Extension,
        routing::{get, post},
        AddExtensionLayer,
        Router,
    },
    learn_axum::{logging::LogService, sample_middle::MyMiddleware, PORT},
    serde::Deserialize,
    serde_json::json,
    std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    },
    tower::layer::layer_fn,
};

#[derive(Debug, Default, Deserialize)]
struct State {
    count: usize,
}

#[tokio::main]
async fn main() {
    console_subscriber::init();
    let shared_state = Arc::new(Mutex::new(State::default()));
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(get_foo_bar))
        .route("/json", get(json))
        .route("/path/:id", get(path))
        .route(
            "/query",
            get({
                let shared_state = Arc::clone(&shared_state);
                move |body| query(body, Arc::clone(&shared_state))
            }),
        )
        .route(
            "/query_json",
            post({
                let shared_state = Arc::clone(&shared_state);
                move |body| query_json(body, Arc::clone(&shared_state))
            }),
        )
        .layer(layer_fn(|service| LogService::new(service, "test")))
        .layer(layer_fn(MyMiddleware::new))
        .layer(AddExtensionLayer::new(shared_state));

    axum::Server::bind(&PORT.parse().unwrap())
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

async fn query(
    Query(params): Query<HashMap<String, String>>,
    _state: Arc<Mutex<State>>,
) -> &'static str {
    dbg!(params);
    "this is query\n"
}

async fn query_json(
    Json(payload): Json<serde_json::Value>,
    state: Arc<Mutex<State>>,
) -> Json<serde_json::Value> {
    dbg!(payload);
    let mut data = state.lock().unwrap();
    data.count += 1;
    Json(json!({ "data": data.count }))
}
