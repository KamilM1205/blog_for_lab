INSERT INTO "testing.Article"(title, file_name, date, author_id, blog_id)
VALUES ($1, $2, $3, $4, $5)
RETURNING $table_fields