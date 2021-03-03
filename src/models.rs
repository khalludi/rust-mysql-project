use super::schema::bj_photos;
use super::schema::breyers_photos;
use super::schema::hd_photos;
use super::schema::talenti_photos;
use super::schema::bj_products;
use super::schema::breyers_products;
use super::schema::talenti_products;
use super::schema::hd_products;
use super::schema::bj_reviews;
use super::schema::breyers_reviews;
use super::schema::hd_reviews;
use super::schema::talenti_reviews;
use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Insertable, Queryable)]
#[table_name="bj_photos"]
pub struct BjPhoto {
    pub photo_id: String,
    pub photo: Vec<u8>
}

#[derive(Insertable, Queryable)]
#[table_name="breyers_photos"]
pub struct BreyersPhoto {
    pub photo_id: String,
    pub photo: Vec<u8>
}


#[derive(Insertable, Queryable)]
#[table_name="hd_photos"]
pub struct HdPhoto {
    pub photo_id: String,
    pub photo: Vec<u8>
}

#[derive(Insertable, Queryable)]
#[table_name="talenti_photos"]
pub struct TalentiPhoto {
    pub photo_id: String,
    pub photo: Vec<u8>
}

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

#[derive(Queryable)]
pub struct BjReviewQueryable {
    pub review_id: i32,
    pub product_id: String,
    pub author: String,
    pub date_posted: NaiveDate,
    pub stars: i32,
    pub title: String,
    pub helpful_yes: i32,
    pub helpful_no: i32,
    pub review_text: String
}

#[derive(Queryable)]
pub struct BreyersReviewQueryable {
    pub review_id: i32,
    pub product_id: String,
    pub author: String,
    pub date_posted: NaiveDate,
    pub stars: i32,
    pub title: String,
    pub helpful_yes: i32,
    pub helpful_no: i32,
    pub review_text: String
}

#[derive(Queryable)]
pub struct HdReviewQueryable {
    pub review_id: i32,
    pub product_id: String,
    pub author: String,
    pub date_posted: NaiveDate,
    pub stars: i32,
    pub title: String,
    pub helpful_yes: i32,
    pub helpful_no: i32,
    pub review_text: String,
    pub taste: f64,
    pub ingredients :f64,
    pub texture: f64,
    pub likes: String
}

#[derive(Queryable)]
pub struct TalentiReviewQueryable {
    pub review_id: i32,
    pub product_id: String,
    pub author: String,
    pub date_posted: NaiveDate,
    pub stars: i32,
    pub title: String,
    pub helpful_yes: f64,
    pub helpful_no: f64,
    pub review_text: String
}

#[derive(Insertable, Deserialize)]
#[table_name="bj_reviews"]
pub struct BjReview {
    pub product_id: String,
    pub author: String,

    #[serde(with = "my_date_format")]
    pub date_posted: NaiveDate,

    pub stars: i32,
    pub title: String,
    pub helpful_yes: i32,
    pub helpful_no: i32,
    pub review_text: String
}

#[derive(Insertable, Deserialize)]
#[table_name="breyers_reviews"]
pub struct BreyersReview {
    pub product_id: String,
    pub author: String,

    #[serde(with = "my_date_format")]
    pub date_posted: NaiveDate,

    pub stars: i32,
    pub title: String,
    pub helpful_yes: i32,
    pub helpful_no: i32,
    pub review_text: String
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name="hd_reviews"]
pub struct HdReview {
    pub product_id: String,
    pub author: String,

    #[serde(with = "my_date_format")]
    pub date_posted: NaiveDate,

    pub stars: i32,
    pub title: String,
    pub helpful_yes: i32,
    pub helpful_no: i32,
    pub review_text: String,

    #[serde(with = "default_dbl_zero")]
    pub taste: f64,

    #[serde(with = "default_dbl_zero")]
    pub ingredients: f64,

    #[serde(with = "default_dbl_zero")]
    pub texture: f64,
    pub likes: String
}

#[derive(Insertable, Deserialize)]
#[table_name="talenti_reviews"]
pub struct TalentiReview {
    pub product_id: String,
    pub author: String,

    #[serde(with = "my_date_format")]
    pub date_posted: NaiveDate,

    pub stars: i32,
    pub title: String,
    pub helpful_yes: f64,
    pub helpful_no: f64,
    pub review_text: String
}

mod my_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";
    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

mod default_dbl_zero {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(0.0)
        } else {
            Ok(s.parse::<f64>().unwrap())
        }
    }
}