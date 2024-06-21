use rocket_routes::crates;
use rocket_routes::rustaceans;

mod models;
mod repository;
mod rocket_routes;
mod schema;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                rustaceans::get_rust_version,
                rustaceans::get_db_conn,
                rustaceans::get_rustacean,
                rustaceans::get_rustaceans,
                rustaceans::create_rustacean,
                rustaceans::update_rustacean,
                rustaceans::delete_rustacean,
                crates::get_crate,
                crates::get_crates,
                crates::create_crate,
                crates::update_crate,
                crates::delete_crate,
            ],
        )
        .attach(rocket_routes::DbConn::init())
        .launch()
        .await;
}
