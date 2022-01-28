use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/", get(get_foo).post(post_foo))
        .route("/", get(get_foo_bar))
        ;
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

