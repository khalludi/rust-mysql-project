use super::schema::bj_products;
use serde::Deserialize;

#[derive(Insertable, Queryable, Deserialize)]
#[table_name="bj_products"]
pub struct Product {
    pub product_id: String,
    pub name: String,
    pub subhead: String,
    pub description: String,
    pub rating: f64,
    pub rating_count: i32,
    pub ingredients: String
}