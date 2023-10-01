CREATE TABLE books
(
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(100) NOT NULL,
    isbn_code       VARCHAR(20)  NOT NULL,
    author          VARCHAR(100) NOT NULL,
    revision_number INT          NOT NULL,
    publisher       VARCHAR(100) NOT NULL
);