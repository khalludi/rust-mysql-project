extern crate icecream_sql;
extern crate diesel;

use self::icecream_sql::*;
use self::models::*;
use self::diesel::prelude::*;
// use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::{fs, io};

const SHOULD_INSERT: bool = false;

fn main() -> Result<(), csv::Error> {

    let connection = establish_connection();
    // display_results(&connection);
    download_bj_photos(&connection);

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
        insert_bj_reviews(&connection)
            .expect("Failed to insert bj_reviews table");
        insert_breyers_reviews(&connection)
            .expect("Failed to insert breyers_reviews table");
        insert_hd_reviews(&connection)
            .expect("Failed to insert hd_reviews table");
        insert_talenti_reviews(&connection)
            .expect("Failed to insert talenti_reviews table");
        insert_bj_photos(&connection)
            .expect("Failed to insert bj_photos table");
    }

    Ok(())
}

fn download_bj_photos(conn: &MysqlConnection) {
    use icecream_sql::schema::bj_photos::dsl::*;

    let results = bj_photos
        .limit(1)
        .load::<BjPhoto>(conn)
        .expect("Error loading bj_photos");

    println!("Displaying {} photos", results.len());
    for photos in results {
        println!("Downloaded {} Photo\n", photos.photo_id);
        let bytes = photos.photo;
        fs::write(photos.photo_id, bytes)
            .expect("Failed to write photo");
    }
}

fn display_results(conn: &MysqlConnection) {
    use icecream_sql::schema::bj_reviews::dsl::*;

    // let results = bj_products.filter(rating.gt(4.0))
    //     .limit(5)
    //     .load::<BjProduct>(conn)
    //     .expect("Error loading products");
    let results = bj_reviews
        .limit(10)
        .load::<BjReviewQueryable>(conn)
        .expect("Error loading products");

    println!("Displaying {} products", results.len());
    for product in results {
        println!("{}, rating: {}\n", product.product_id, product.stars);
        println!("{}", product.review_text);
        println!("-------------\n");
    }
}

fn insert_bj_photos(conn: &MysqlConnection) -> io::Result<()> {
    let entries = fs::read_dir("archive/bj/images")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    
    let mut photos_vec: Vec<BjPhoto> = vec![];
    for e in &entries {
        let bytes = std::fs::read(e);
        let this_photo = BjPhoto {
            photo_id: e.file_name().ok_or(0).unwrap().to_str().ok_or(0).unwrap().to_string(), 
            photo: bytes.unwrap_or_default()
        };
        photos_vec.push(this_photo);
    }

    create_bj_photos(conn, photos_vec);
    
    Ok(())
}

fn insert_bj_reviews(conn: &MysqlConnection) -> Result<(), csv::Error> {
    let path = Path::new("/Users/khalid/prog/rust_prog/icecream_sql/archive/bj/reviews.csv");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read CSV as Vector of Reviews
    let mut reader = csv::Reader::from_reader(file);
    let mut reviews_vec: Vec<BjReview> = vec![];

    for record in reader.deserialize() {
        let record: BjReview = record?;
        reviews_vec.push(record);
    }

    create_bj_reviews(&conn, reviews_vec);

    Ok(())
}

fn insert_breyers_reviews(conn: &MysqlConnection) -> Result<(), csv::Error> {
    let path = Path::new("/Users/khalid/prog/rust_prog/icecream_sql/archive/breyers/reviews.csv");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read CSV as Vector of Reviews
    let mut reader = csv::Reader::from_reader(file);
    let mut reviews_vec: Vec<BreyersReview> = vec![];

    for record in reader.deserialize() {
        let record: BreyersReview = record?;
        reviews_vec.push(record);
    }

    create_breyers_reviews(&conn, reviews_vec);

    Ok(())
}

fn insert_hd_reviews(conn: &MysqlConnection) -> Result<(), csv::Error> {
    let path = Path::new("/Users/khalid/prog/rust_prog/icecream_sql/archive/hd/reviews.csv");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read CSV as Vector of Reviews
    let mut reader = csv::Reader::from_reader(file);
    let mut reviews_vec: Vec<HdReview> = vec![];

    for record in reader.deserialize() {
        let record: HdReview = record?;
        reviews_vec.push(record);
        // println!("{:?}", record);
    }

    create_hd_reviews(&conn, reviews_vec);

    Ok(())
}

fn insert_talenti_reviews(conn: &MysqlConnection) -> Result<(), csv::Error> {
    let path = Path::new("/Users/khalid/prog/rust_prog/icecream_sql/archive/talenti/reviews.csv");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read CSV as Vector of Reviews
    let mut reader = csv::Reader::from_reader(file);
    let mut reviews_vec: Vec<TalentiReview> = vec![];

    for record in reader.deserialize() {
        let record: TalentiReview = record?;
        reviews_vec.push(record);
    }

    create_talenti_reviews(&conn, reviews_vec);

    Ok(())
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