#[cfg(not(target_arch = "wasm32"))]
use axum::{
    routing::{get, post},
    Router,
};
use wrpc::rpc;

#[rpc(post("/api/hello"))]
pub async fn hello(req_body: String) -> String {
    format!("Hello, {req_body}!")
}

#[rpc(get("/api/hello"))]
pub async fn test() -> &'static str {
    "pong"
}

#[cfg(not(target_arch = "wasm32"))]
pub fn routes() -> Router {
    Router::new()
        .route("/test", get(test))
        .route("/hello", post(hello))
}
