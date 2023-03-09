-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE "posts" (
                         "id" Uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
                         "title" varchar NOT NULL,
                         "content_html" varchar NOT NULL,
                         "created_at" timestamp NOT NULL DEFAULT now(),
                         "updated_at" timestamp NOT NULL DEFAULT now()
);

