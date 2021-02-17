extern crate icecream_sql;
extern crate diesel;

use self::icecream_sql::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use icecream_sql::schema::bj_products::dsl::*;

    let connection = establish_connection();
    let results = bj_products.filter(rating.gt(4.0))
        .limit(5)
        .load::<Product>(&connection)
        .expect("Error loading products");

    println!("Displaying {} products", results.len());
    for product in results {
        println!("{}", product.name);
        println!("-------------\n");
        println!("{}", product.description);
    }
}