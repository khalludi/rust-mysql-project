extern crate icecream_sql;
extern crate diesel;

use self::icecream_sql::*;
use self::models::*;
use self::diesel::prelude::*;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

const SHOULD_INSERT: bool = true;

fn main() -> Result<(), csv::Error> {

    let connection = establish_connection();
    display_results(&connection);

    // File IO
    if SHOULD_INSERT {
        insert_bj_products(&connection)
            .expect("Failed to insert bj_products table");
        insert_breyers_products(&connection)
            .expect("Failed to insert bj_products table");
        insert_hd_products(&connection)
            .expect("Failed to insert bj_products table");
        insert_talenti_products(&connection)
            .expect("Failed to insert bj_products table");
    }

    Ok(())
}

fn display_results(conn: &MysqlConnection) {
    use icecream_sql::schema::bj_products::dsl::*;

    let results = bj_products.filter(rating.gt(4.0))
        .limit(5)
        .load::<BjProduct>(conn)
        .expect("Error loading products");

    println!("Displaying {} products", results.len());
    for product in results {
        println!("{}, rating: {}\n", product.name, product.rating);
        println!("{}", product.description);
        println!("-------------\n");
    }
}

fn insert_bj_products(conn: &MysqlConnection) -> Result<(), csv::Error> {
    let path = Path::new("/Users/khalid/prog/rust_prog/icecream_sql/archive/bj/products.csv");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read CSV as Vector of Products
    let mut reader = csv::Reader::from_reader(file);
    let mut product_vec: Vec<BjProduct> = vec![];

    for record in reader.deserialize() {
        let record: BjProduct = record?;
        product_vec.push(record);
    }

    create_bj_posts(&conn, product_vec);

    Ok(())
}

fn insert_breyers_products(conn: &MysqlConnection) -> Result<(), csv::Error> {
    let path = Path::new("/Users/khalid/prog/rust_prog/icecream_sql/archive/breyers/products.csv");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read CSV as Vector of Products
    let mut reader = csv::Reader::from_reader(file);
    let mut product_vec: Vec<BreyersProduct> = vec![];

    for record in reader.deserialize() {
        let record: BreyersProduct = record?;
        product_vec.push(record);
    }

    create_breyers_posts(&conn, product_vec);

    Ok(())
}

fn insert_hd_products(conn: &MysqlConnection) -> Result<(), csv::Error> {
    let path = Path::new("/Users/khalid/prog/rust_prog/icecream_sql/archive/hd/products.csv");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read CSV as Vector of Products
    let mut reader = csv::Reader::from_reader(file);
    let mut product_vec: Vec<HdProduct> = vec![];

    for record in reader.deserialize() {
        let record: HdProduct = record?;
        product_vec.push(record);
    }

    create_hd_posts(&conn, product_vec);

    Ok(())
}

fn insert_talenti_products(conn: &MysqlConnection) -> Result<(), csv::Error> {
    let path = Path::new("/Users/khalid/prog/rust_prog/icecream_sql/archive/talenti/products.csv");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read CSV as Vector of Products
    let mut reader = csv::Reader::from_reader(file);
    let mut product_vec: Vec<TalentiProduct> = vec![];

    for record in reader.deserialize() {
        let record: TalentiProduct = record?;
        product_vec.push(record);
    }

    create_talenti_posts(&conn, product_vec);

    Ok(())
}