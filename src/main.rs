use {
    axum::{
        body::Body,
        extract::{Json, Path, Query},
        http::Request,
        routing::get,
        response::Response,
        // response::Json,
        Router,
    },
    futures::future::BoxFuture,
    serde_json::json,
    std::{
        collections::HashMap,
        task::{Context, Poll},
    },
    tower::{layer::layer_fn, Service},
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
        .route("/m", get(|| async { dbg!(); }))
        .layer(layer_fn(|inner| MyMiddleware { inner }))
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

#[derive(Clone, Debug)]
struct MyMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for MyMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        println!("`MyMiddleware` called!");

        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let res: Response = inner.call(req).await?;
            println!("`MyMiddleware` received the response");
            Ok(res)
        })
    }
}
