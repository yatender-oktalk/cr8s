use rocket_db_pools::Database;

mod models;
mod repository;
mod schema;
mod rocket_routes;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![rocket_routes::rustaceans::get_rust_version, rocket_routes::rustaceans::get_db_conn])
        .attach(DbConn::init())
        .launch()
        .await;
}



#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);
