-- Add migration script here
ALTER TABLE "project" ADD COLUMN user_id INT NOT NULL;
ALTER TABLE "project" ADD CONSTRAINT project_user_id_fkey FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE;