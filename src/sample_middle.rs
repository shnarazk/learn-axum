/// https://docs.rs/tower/0.4.6/tower/trait.Layer.html#log
use {
    axum::{body::Body, http::Request, response::Response},
    futures::future::BoxFuture,
    std::task::{Context, Poll},
    tower::Service,
};

#[derive(Clone, Debug)]
pub struct MyMiddleware<S> {
    inner: S,
}

impl<S> MyMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
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
