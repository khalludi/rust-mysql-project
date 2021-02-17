CREATE TABLE bj_products (
    product_id VARCHAR(30) NOT NULL,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(400) NOT NULL,
    rating REAL NOT NULL,
    rating_count INT NOT NULL,
    ingredients VARCHAR(500) NOT NULL,
    PRIMARY KEY(product_id)
);