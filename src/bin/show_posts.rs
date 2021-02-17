extern crate icecream_sql;
extern crate diesel;

use self::icecream_sql::*;
use self::models::*;
use self::diesel::prelude::*;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

const SHOULD_INSERT: bool = false;

fn main() -> Result<(), csv::Error> {
    use icecream_sql::schema::bj_products::dsl::*;

    let connection = establish_connection();
    let results = bj_products.filter(rating.gt(4.0))
        .limit(5)
        .load::<Product>(&connection)
        .expect("Error loading products");

    println!("Displaying {} products", results.len());
    for product in results {
        println!("{}, rating: {}\n", product.name, product.rating);
        println!("{}", product.description);
        println!("-------------\n");
    }

    // // File IO
    // let path = Path::new("/Users/khalid/prog/rust_prog/icecream_sql/archive/bj/products.csv");
    // let display = path.display();

    // // Open the path in read-only mode, returns `io::Result<File>`
    // let file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // // Read CSV as Vector of Products
    // let mut reader = csv::Reader::from_reader(file);
    // let mut product_vec: Vec<Product> = vec![];

    // for record in reader.deserialize() {
    //     let record: Product = record?;
    //     product_vec.push(record);
    //     // println!("Name: {}", record.name);
    // }

    // if SHOULD_INSERT {
    //     create_posts(&connection, product_vec);
    // }

    Ok(())
}