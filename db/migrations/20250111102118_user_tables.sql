-- Active: 1736456114282@@127.0.0.1@5432@nails@public
-- migrate:up
CREATE TABLE users (
    id SERIAL PRIMARY KEY, 
    email VARCHAR NOT NULL UNIQUE, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO users(email) VALUES('test4@test1.com');
INSERT INTO users(email) VALUES('test5@test1.com');
INSERT INTO users(email) VALUES('test6@test1.com');

-- migrate:down
DROP TABLE users;