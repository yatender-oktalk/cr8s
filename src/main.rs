use repository::RustaceanRepository;
use rocket::http::hyper::server::conn::Connection;
use rocket_db_pools::{Connection, Database};

mod models;
mod repository;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![get_rust_version])
        .attach(DbConn::init())
        .launch()
        .await;
}

#[rocket::get("/version")]
fn get_rust_version() -> String {
    "0.1.0".to_string()
}

#[rocket::get("/db")]
async fn get_db_conn(db: Connection<DbConn>()) {
    RustaceanRepository::find_many(&mut db, 5, 0).await
}

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);
