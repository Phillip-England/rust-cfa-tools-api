use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn cors() -> CorsLayer {
  // cors middleware
  let cors = CorsLayer::new()
  .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
  .allow_origin(Any);
  cors
}