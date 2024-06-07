use rocket_db_pools::Database;
use rocket_routes::rustaceans;

mod models;
mod repository;
mod rocket_routes;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![rustaceans::get_rust_version, rustaceans::get_db_conn],
        )
        .attach(DbConn::init())
        .launch()
        .await;
}

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);
