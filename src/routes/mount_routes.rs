use crate::connect_mongo;
use crate::AppState;
use crate::cors;
use crate::create_user;

use axum::{routing::post, Router};
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
  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

}
