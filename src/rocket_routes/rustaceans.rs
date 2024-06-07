use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_db_pools::Connection;

use crate::models::NewRustacean;
use crate::repository::RustaceanRepository;
use crate::DbConn;

#[rocket::get("/version")]
pub async fn get_rust_version() -> String {
    "0.1.0".to_string()
}

#[rocket::get("/db")]
pub async fn get_db_conn(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_many(&mut db, 5, 0)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::get("/rustacians")]
pub async fn get_rustacians(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    todo!()
}

#[rocket::post("/rustacians", format = "json", data = "<new_rustacean>")]
pub fn create_rustacian(
    mut db: Connection<DbConn>,
    new_rustacean: Json<NewRustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    todo!()
}

#[rocket::put("/rustacians/<id>", format = "json", data = "<rustacian>")]
pub async fn update_rustacian(
    mut db: Connection<DbConn>,
    id: i32,
    rustacian: Json<Rustacian>,
) -> Result<Value, Custom<Value>> {
    todo!()
}
