use crate::connect_mongo;
use crate::cors;
use crate::create_user;
use crate::AppState;

use axum::{routing::post, Router};
use std::env;
use std::sync::Arc;

pub async fn mount_routes() {
  // pulling in resources
  let db = connect_mongo().await;
  let app_state: Arc<AppState> = Arc::new(AppState { db: db });
  let cors = cors();

  // creating routes
  let app: Router = Router::new()
    .route("/user", post(create_user))
    .layer(cors)
    .with_state(app_state);

  // serving our app
  let port = env::var("PORT").unwrap();
  let address = format!("0.0.0.0:{}", port);
  axum::Server::bind(&address.parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
