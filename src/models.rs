use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable, AsChangeset, serde::Serialize)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub age: i32,
    pub email: String,
}

#[derive(Debug, Queryable)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

