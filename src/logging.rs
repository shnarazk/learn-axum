/// https://docs.rs/tower/0.4.6/tower/trait.Layer.html#log
use {
    axum::{body::Body, http::Request, response::Response},
    futures::future::BoxFuture,
    std::task::{Context, Poll},
    tower::{Layer, Service},
};

#[derive(Clone, Debug)]
pub struct LogLayer {
    target: &'static str,
}

#[derive(Clone, Debug)]
pub struct LogService<S> {
    target: &'static str,
    service: S,
    log: Vec<String>,
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;
    fn layer(&self, service: S) -> Self::Service {
        LogService {
            target: self.target,
            service,
            log: Vec::new(),
        }
    }
}

impl<S> LogService<S> {
    pub fn new(service: S, target: &'static str) -> Self {
        LogService {
            target,
            service,
            log: Vec::new(),
        }
    }
}
impl<S> Service<Request<Body>> for LogService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.log.push("check poll_ready".to_string());
        self.service.poll_ready(cx)
    }
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        self.log
            .push(format!("Log{}:called {:?}", self.target, req));
        dbg!(&self.log);
        let clone = self.service.clone();
        let mut inner = std::mem::replace(&mut self.service, clone);
        Box::pin(async move {
            let res: Response = inner.call(req).await?;
            Ok(res)
        })
    }
}
