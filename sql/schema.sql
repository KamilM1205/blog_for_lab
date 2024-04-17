CREATE TABLE IF NOT EXISTS "testing.Category" (
	"id" bigint GENERATED ALWAYS AS IDENTITY NOT NULL UNIQUE,
	"blog_id" serial NOT NULL,
	"name" bigint NOT NULL UNIQUE,
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "testing.Category_Type" (
	"id" bigint GENERATED ALWAYS AS IDENTITY NOT NULL UNIQUE,
	"name" text NOT NULL UNIQUE,
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "testing.Comment" (
	"id" bigint GENERATED ALWAYS AS IDENTITY NOT NULL UNIQUE,
	"article_id" serial NOT NULL,
	"author_id" bigint NOT NULL,
	"text" text NOT NULL,
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "testing.Blog" (
	"id" bigint GENERATED ALWAYS AS IDENTITY NOT NULL UNIQUE,
	"title" text NOT NULL,
	"description" text,
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "testing.Author" (
	"id" bigint GENERATED ALWAYS AS IDENTITY NOT NULL UNIQUE,
	"nickname" text NOT NULL,
	"name" text NOT NULL,
	"surname" text NOT NULL,
	"date" date NOT NULL,
	"email" text NOT NULL,
	"phone" bigint NOT NULL,
	"image" text NOT NULL,
	"birthday" date NOT NULL,
	"password" text NOT NULL,
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "testing.Article" (
	"id" bigint GENERATED ALWAYS AS IDENTITY NOT NULL UNIQUE,
	"title" text NOT NULL,
	"file_name" text NOT NULL,
	"date" text NOT NULL,
	"author_id" bigint NOT NULL,
	"blog_id" bigint NOT NULL,
	PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "testing.Articles" (
	"id" bigint GENERATED ALWAYS AS IDENTITY NOT NULL UNIQUE,
	"blog_id" bigint NOT NULL,
	"article_id" bigint NOT NULL,
	PRIMARY KEY ("id")
);

ALTER TABLE "testing.Category" ADD CONSTRAINT "Category_fk1" FOREIGN KEY ("blog_id") REFERENCES "testing.Blog"("id");

ALTER TABLE "testing.Category" ADD CONSTRAINT "Category_fk2" FOREIGN KEY ("name") REFERENCES "testing.Category_Type"("id");

ALTER TABLE "testing.Comment" ADD CONSTRAINT "Comment_fk1" FOREIGN KEY ("article_id") REFERENCES "testing.Article"("id");

ALTER TABLE "testing.Comment" ADD CONSTRAINT "Comment_fk2" FOREIGN KEY ("author_id") REFERENCES "testing.Author"("id");

ALTER TABLE "testing.Article" ADD CONSTRAINT "Article_fk4" FOREIGN KEY ("author_id") REFERENCES "testing.Author"("id");

ALTER TABLE "testing.Article" ADD CONSTRAINT "Article_fk5" FOREIGN KEY ("blog_id") REFERENCES "testing.Blog"("id");

ALTER TABLE "testing.Articles" ADD CONSTRAINT "Articles_fk1" FOREIGN KEY ("blog_id") REFERENCES "testing.Blog"("id");

ALTER TABLE "testing.Articles" ADD CONSTRAINT "Articles_fk2" FOREIGN KEY ("article_id") REFERENCES "testing.Article"("id");