use crate::AppState;
// use crate::CreateUserRequest;
use crate::Validator;

use axum::{
  extract::Json,
  extract::State,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use bcrypt::hash;
use mongodb::{bson::doc, bson::Document};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
  pub username: String,
  pub password: String,
}

#[derive(Serialize, Debug)]
pub struct CreateUserResponse {
  pub username: String,
}

pub async fn create_user(
  State(state): State<Arc<AppState>>,
  Json(payload): Json<CreateUserRequest>,
) -> Response {
  // extracting request body
  let mut username: String = payload.username;
  let password: String = payload.password;

  // all usernames are lowercase by default
  username = username.to_lowercase();

  // getting our mongo collection
  let collection = state.db.collection::<Document>("users");

  // checking if user already exists
  let filter = doc! {"username": &username};
  let _user_doc: std::option::Option<Document> = match collection.find_one(filter, None).await {
    Ok(Some(_user_doc)) => {
      return (StatusCode::BAD_REQUEST, "user already exists".to_owned()).into_response();
    }
    Ok(None) => None,
    Err(_e) => {
      return (StatusCode::INTERNAL_SERVER_ERROR, "server error".to_owned()).into_response();
    }
  };

  // validating username
  let username_validator: Validator<String> = Validator::new(username)
    .max_length(
      32,
      "username may contain between 5 and 32 characters".to_owned(),
    )
    .min_length(
      5,
      "username may contain between 5 and 32 characters".to_owned(),
    )
    .whitelist(
      "-_".to_owned(),
      "username may contain letters, numbers, dashes, and underscores".to_owned(),
    )
    .run_validation();

  // validating password
  let password_validator: Validator<String> = Validator::new(password)
    .max_length(
      64,
      "password may contain between 8 and 64 characters".to_owned(),
    )
    .min_length(
      8,
      "password may contain between 8 and 64 characters".to_owned(),
    )
    .whitelist(
      " !@#$%^&*()-_+={[}]?".to_owned(),
      "password may only contain common symbols".to_owned(),
    )
    .run_validation();

  // checking for username validation errors
  if username_validator.validation_errors.len() > 0 {
    return (
      StatusCode::BAD_REQUEST,
      username_validator.get_error_message(),
    )
      .into_response();
  }

  // checking for password validation errors
  if password_validator.validation_errors.len() > 0 {
    return (
      StatusCode::BAD_REQUEST,
      password_validator.get_error_message(),
    )
      .into_response();
  }

  // encrypting password
  let hashed_password = hash(password_validator.value, 10).unwrap();

  // inserting user into colleciton
  let doc = doc! {"username": username_validator.value.to_owned(), "password": hashed_password};
  collection.insert_one(doc, None).await.unwrap();

  // creating our response
  let response: CreateUserResponse = CreateUserResponse {
    username: username_validator.value.to_owned(),
  };

  // returning json to client
  (StatusCode::CREATED, Json(response)).into_response()
}
