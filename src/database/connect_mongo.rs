use mongodb::{
  options::{ClientOptions, ServerApi, ServerApiVersion},
  Client, Database,
};
use std::env;

pub async fn connect_mongo() -> Database {
  //connecting to mongo db
  let mongo_uri: String = env::var("MONGO_URI").unwrap();
  let mut client_options: ClientOptions = ClientOptions::parse(mongo_uri).await.unwrap();
  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.max_pool_size = Some(10);
  client_options.server_api = Some(server_api);
  let client = Client::with_options(client_options).unwrap();
  let db = client.database(&env::var("DB_NAME").unwrap());
  db
}
