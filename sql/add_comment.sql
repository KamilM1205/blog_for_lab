INSERT INTO "testing.Comment"(article_id, author_id, text)
VALUES ($1, $2, $3)
RETURNING $table_fields;