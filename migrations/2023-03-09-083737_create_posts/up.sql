-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE "posts" (
                         "id" Uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
                         "content_id" varchar NOT NULL,
                         "title" varchar NOT NULL,
                         "content_html" varchar NOT NULL,
                         "created_at" timestamp NOT NULL DEFAULT now(),
                         "updated_at" timestamp NOT NULL DEFAULT now()
);

CREATE FUNCTION set_update_time() RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'UPDATE') THEN
        NEW.updated_at := now();
return NEW;
END IF;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trig_update_post BEFORE UPDATE ON "posts" FOR EACH ROW EXECUTE PROCEDURE set_update_time();
