table! {
    bj_products (product_id) {
        product_id -> Varchar,
        name -> Varchar,
        subhead -> Varchar,
        description -> Varchar,
        rating -> Double,
        rating_count -> Integer,
        ingredients -> Varchar,
    }
}

table! {
    bj_reviews (review_id) {
        review_id -> Integer,
        product_id -> Varchar,
        author -> Varchar,
        date_posted -> Date,
        stars -> Integer,
        title -> Varchar,
        helpful_yes -> Integer,
        helpful_no -> Integer,
        review_text -> Varchar,
    }
}

table! {
    breyers_products (product_id) {
        product_id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        rating -> Double,
        rating_count -> Integer,
        ingredients -> Varchar,
    }
}

table! {
    hd_products (product_id) {
        product_id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        rating -> Double,
        rating_count -> Integer,
        ingredients -> Varchar,
    }
}

table! {
    talenti_products (product_id) {
        product_id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        rating -> Double,
        rating_count -> Integer,
        ingredients -> Varchar,
    }
}

joinable!(bj_reviews -> bj_products (product_id));

allow_tables_to_appear_in_same_query!(
    bj_products,
    bj_reviews,
    breyers_products,
    hd_products,
    talenti_products,
);
