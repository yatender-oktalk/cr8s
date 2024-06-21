use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_db_pools::Connection;

use crate::models::Crate;
use crate::models::NewCrate;
use crate::repository::CrateRepository;

use super::DbConn;

// We will fetch the limit and offset from the query params from the request.
#[rocket::get("/crates?<limit>&<offset>")]
pub async fn get_crates(
    mut db: Connection<DbConn>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Value, Custom<Value>> {
    let offset = offset.unwrap_or(0);
    let limit = limit.unwrap_or(10);
    CrateRepository::find_many(&mut db, limit, offset)
        .await
        .map(|crates| json!(crates))
        .map_err(|_| Custom(Status::NotFound, json!("No crates available")))
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    CrateRepository::find_one(&mut db, id)
        .await
        .map(|crates| json!(crates))
        .map_err(|_| Custom(Status::NotFound, json!("Crate not found")))
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConn>,
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|crate_data| Custom(Status::Created, json!(crate_data)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::put("/crates/<id>", format = "json", data = "<rustacian>")]
pub async fn update_crate(
    mut db: Connection<DbConn>,
    id: i32,
    rustacian: Json<Crate>,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::update(&mut db, id, rustacian.into_inner())
        .await
        .map(|rustacean| Custom(Status::Ok, json!(rustacean)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|_| Custom(Status::Ok, json!({ "status": "Deleted successfully" })))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}
