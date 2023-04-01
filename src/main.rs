mod database;
use database::connect_mongo::connect_mongo;

mod service;
use service::validator::Validator;

mod middleware;
use middleware::cors::cors;

mod models;
use models::app_state::AppState;
use models::create_user_request::CreateUserRequest;

mod routes;
use routes::mount_routes::mount_routes;
use routes::create_user::create_user;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
  dotenv().ok();
  mount_routes().await;
}
