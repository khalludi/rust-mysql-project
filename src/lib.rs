pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::BjProduct;
use self::models::BreyersProduct;
use self::models::HdProduct;
use self::models::TalentiProduct;
use self::models::BjReview;
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

pub fn create_bj_reviews<'a>(conn: &MysqlConnection, vec: Vec<BjReview>) {
    use schema::bj_reviews;

    diesel::insert_into(bj_reviews::table)
        .values(vec)
        .execute(conn)
        .expect("Error saving bj reviews");
}

pub fn create_bj_posts<'a>(conn: &MysqlConnection, vec: Vec<BjProduct>) {
    use schema::bj_products;

    diesel::insert_into(bj_products::table)
        .values(vec)
        .execute(conn)
        .expect("Error saving bj posts");
}

pub fn create_breyers_posts<'a>(conn: &MysqlConnection, vec: Vec<BreyersProduct>) {
    use schema::breyers_products;

    diesel::insert_into(breyers_products::table)
        .values(vec)
        .execute(conn)
        .expect("Error saving breyers posts");
}

pub fn create_hd_posts<'a>(conn: &MysqlConnection, vec: Vec<HdProduct>) {
    use schema::hd_products;

    diesel::insert_into(hd_products::table)
        .values(vec)
        .execute(conn)
        .expect("Error saving hd posts");
}

pub fn create_talenti_posts<'a>(conn: &MysqlConnection, vec: Vec<TalentiProduct>) {
    use schema::talenti_products;

    diesel::insert_into(talenti_products::table)
        .values(vec)
        .execute(conn)
        .expect("Error saving talenti posts");
}