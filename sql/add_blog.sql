INSERT INTO "testing.Blog"(title, description)
VALUES ($1, $2)
RETURNING $table_fields;