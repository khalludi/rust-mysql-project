-- Product Tables
CREATE TABLE talenti_products (
    product_id VARCHAR(30) NOT NULL,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(2000) NOT NULL,
    rating REAL NOT NULL,
    rating_count INT NOT NULL,
    ingredients VARCHAR(2000) NOT NULL,
    PRIMARY KEY(product_id)
);

CREATE TABLE hd_products (
    product_id VARCHAR(30) NOT NULL,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(2000) NOT NULL,
    rating REAL NOT NULL,
    rating_count INT NOT NULL,
    ingredients VARCHAR(2000) NOT NULL,
    PRIMARY KEY(product_id)
);

CREATE TABLE breyers_products (
    product_id VARCHAR(30) NOT NULL,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(2000) NOT NULL,
    rating REAL NOT NULL,
    rating_count INT NOT NULL,
    ingredients VARCHAR(2000) NOT NULL,
    PRIMARY KEY(product_id)
);

CREATE TABLE bj_products (
    product_id VARCHAR(30) NOT NULL,
    name VARCHAR(100) NOT NULL,
    subhead VARCHAR(200) NOT NULL,
    description VARCHAR(400) NOT NULL,
    rating REAL NOT NULL,
    rating_count INT NOT NULL,
    ingredients VARCHAR(500) NOT NULL,
    PRIMARY KEY(product_id)
);

-- Review Tables  
CREATE TABLE bj_reviews (
    review_id INT NOT NULL AUTO_INCREMENT,
    product_id VARCHAR(30) NOT NULL,
    author VARCHAR(100) NOT NULL DEFAULT "",
    date_posted DATE NOT NULL,
    stars INT NOT NULL,
    title VARCHAR(200) NOT NULL DEFAULT "",
    helpful_yes INT NOT NULL DEFAULT 0,
    helpful_no INT NOT NULL DEFAULT 0,
    review_text VARCHAR(2000) NOT NULL DEFAULT "",
    PRIMARY KEY(review_id),
    FOREIGN KEY(product_id) REFERENCES bj_products(product_id)
);
