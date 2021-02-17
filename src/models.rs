#[derive(Queryable)]
pub struct Product {
    pub product_id: String,
    pub name: String,
    pub description: String,
    pub rating: f64,
    pub rating_count: i32,
    pub ingredients: String
}