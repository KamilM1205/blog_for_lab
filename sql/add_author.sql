INSERT INTO "testing.Author"(nickname, name, surname, date, email, phone, image, birthday, password)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
RETURNING $table_fields