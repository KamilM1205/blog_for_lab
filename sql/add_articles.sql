INSERT INTO "testing.Articles"(blog_id, article_id)
VALUES ($1, $2)
RETURNING $table_fields;