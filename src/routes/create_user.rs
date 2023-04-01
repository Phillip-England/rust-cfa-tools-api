use crate::AppState;
use crate::CreateUserRequest;
use crate::Validator;

use std::sync::Arc;
use axum::{extract::Json, extract::State, http::StatusCode, response::{IntoResponse, Response}};
use mongodb::{bson::Document, bson::doc};


pub async fn create_user(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<CreateUserRequest>,
) -> Response {

  // extracting request body
  let username: String = payload.username;
  let password: String = payload.password;

  // getting our mongo collection
  let collection = state.db.collection::<Document>("users");

  // checking if user already exists
  let filter = doc! {"username": &username};
  let _user_doc: std::option::Option<Document> =  match collection.find_one(filter, None).await {
    Ok(Some(_user_doc)) => {
      return (StatusCode::BAD_REQUEST, "user already exists".to_owned()).into_response();
    },
    Ok(None) => None,
    Err(_e) => {
      return (StatusCode::INTERNAL_SERVER_ERROR, "server error".to_owned()).into_response();
    }
  };

  // validating username
  let username_validator: Validator<String> = Validator::new(username)
    .max_length(32);

  dbg!(&username_validator);

  // inserting user into colleciton
  // let doc = doc! {"username": username, "password": password};
  // collection.insert_one(doc, None).await.unwrap();

  // returning json to client
  (StatusCode::CREATED, "user created".to_owned()).into_response()
}