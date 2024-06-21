use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_db_pools::Connection;

use crate::models::NewRustacean;
use crate::models::Rustacean;
use crate::repository::RustaceanRepository;

use super::DbConn;

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

// We will fetch the limit and offset from the query params from the request.
#[rocket::get("/rustaceans?<limit>&<offset>")]
pub async fn get_rustaceans(
    mut db: Connection<DbConn>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Value, Custom<Value>> {
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(10);
    RustaceanRepository::find_many(&mut db, limit, offset)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|_| Custom(Status::NotFound, json!("Rustacean not found")))
}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_one(&mut db, id)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::NotFound, json!("Rustacean not found")))
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    mut db: Connection<DbConn>,
    new_rustacean: Json<NewRustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut db, new_rustacean.into_inner())
        .await
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacian>")]
pub async fn update_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    rustacian: Json<Rustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::update(&mut db, id, rustacian.into_inner())
        .await
        .map(|rustacean| Custom(Status::Ok, json!(rustacean)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::delete(&mut db, id)
        .await
        .map(|_| Custom(Status::Ok, json!({ "status": "Deleted successfully" })))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}
