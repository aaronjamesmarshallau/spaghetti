-- Add `tags` to `recipe`
ALTER TABLE recipe
ADD COLUMN "tags" TEXT[] NOT NULL DEFAULT '{}'; -- Utilise TEXT[] instead of VARCHAR[] due to limitations with PG and diesel
