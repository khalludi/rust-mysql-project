pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::Product;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_posts<'a>(conn: &MysqlConnection, vec: Vec<Product>) {
    use schema::bj_products;

    diesel::insert_into(bj_products::table)
        .values(vec)
        .execute(conn)
        .expect("Error saving new posts");
}