use serde::Deserialize;

  // creating a struct for create_user_request_body
  #[derive(Deserialize, Debug)]
  pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
  }