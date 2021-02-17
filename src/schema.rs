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
    breyers_reviews (review_id) {
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

table! {
    talenti_reviews (review_id) {
        review_id -> Integer,
        product_id -> Varchar,
        author -> Varchar,
        date_posted -> Date,
        stars -> Integer,
        title -> Varchar,
        helpful_yes -> Double,
        helpful_no -> Double,
        review_text -> Varchar,
    }
}

joinable!(bj_reviews -> bj_products (product_id));
joinable!(breyers_reviews -> breyers_products (product_id));
joinable!(talenti_reviews -> talenti_products (product_id));

allow_tables_to_appear_in_same_query!(
    bj_products,
    bj_reviews,
    breyers_products,
    breyers_reviews,
    hd_products,
    talenti_products,
    talenti_reviews,
);
