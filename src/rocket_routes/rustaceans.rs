
use rocket_db_pools::{Connection, Database};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::json;
use rocket::serde::json::Value;

use crate::repository::RustaceanRepository;
use crate::DbConn;


#[rocket::get("/version")]
pub async fn get_rust_version() -> String {
    "0.1.0".to_string()
}

#[rocket::get("/db")]
pub async fn get_db_conn(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_many(&mut db, 5, 0).await
    .map(|rustacean| json!(rustacean))
    .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}