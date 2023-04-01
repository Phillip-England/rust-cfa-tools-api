use mongodb::Database;

#[derive(Debug)]
pub struct AppState {
  pub db: Database,
}
