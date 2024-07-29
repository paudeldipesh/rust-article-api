CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL
);

CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    created_by INT4 NOT NULL,
    created_on TIMESTAMPTZ,
    FOREIGN KEY (created_by) REFERENCES users(id)
);
