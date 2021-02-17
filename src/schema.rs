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

allow_tables_to_appear_in_same_query!(
    bj_products,
    breyers_products,
    hd_products,
    talenti_products,
);
