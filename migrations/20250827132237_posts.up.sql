CREATE TABLE posts (
    id VARCHAR,
    title VARCHAR,
    format VARCHAR,
    created_on TIMESTAMP,
    updated_on TIMESTAMP
);

INSERT INTO posts (id, title, format)
VALUES 
    ('post-1', 'Post 1', 'plaintext'),
    ('post-2', 'Post 2', 'html');
