CREATE TABLE posts (
    id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    format VARCHAR NOT NULL,
    created_on TIMESTAMP NOT NULL,
    updated_on TIMESTAMP,
    tags VARCHAR[]
);

INSERT INTO posts (id, title, format, created_on, updated_on, tags)
VALUES 
    ('post-1', 'Post 1', 'plaintext', '2025-09-10 20:45:10', '2025-09-10 20:45:10', '{test1, test2}'),
    ('post-2', 'Post 2', 'html', '2025-09-10 20:45:10', '2025-09-10 20:45:10', '{test1, test2}');
