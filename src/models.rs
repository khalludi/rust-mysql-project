use super::schema::bj_products;
use super::schema::breyers_products;
use super::schema::talenti_products;
use super::schema::hd_products;
use serde::Deserialize;

#[derive(Insertable, Queryable, Deserialize)]
#[table_name="bj_products"]
pub struct BjProduct {
    pub product_id: String,
    pub name: String,
    pub subhead: String,
    pub description: String,
    pub rating: f64,
    pub rating_count: i32,
    pub ingredients: String
}

#[derive(Insertable, Queryable, Deserialize)]
#[table_name="breyers_products"]
pub struct BreyersProduct {
    pub product_id: String,
    pub name: String,
    pub description: String,
    pub rating: f64,
    pub rating_count: i32,
    pub ingredients: String
}

#[derive(Insertable, Queryable, Deserialize)]
#[table_name="hd_products"]
pub struct HdProduct {
    pub product_id: String,
    pub name: String,
    pub description: String,
    pub rating: f64,
    pub rating_count: i32,
    pub ingredients: String
}

#[derive(Insertable, Queryable, Deserialize)]
#[table_name="talenti_products"]
pub struct TalentiProduct {
    pub product_id: String,
    pub name: String,
    pub description: String,
    pub rating: f64,
    pub rating_count: i32,
    pub ingredients: String
}