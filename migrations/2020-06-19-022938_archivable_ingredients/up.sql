-- Add `archived` to `recipe_ingredient`
ALTER TABLE recipe_ingredient
ADD COLUMN "archived" BOOLEAN NOT NULL DEFAULT FALSE;

-- Add `archived` to `ingredient`
ALTER TABLE ingredient
ADD COLUMN "archived" BOOLEAN NOT NULL DEFAULT FALSE;

-- Add `archived` to `recipe`
ALTER TABLE recipe
ADD COLUMN "archived" BOOLEAN NOT NULL DEFAULT FALSE;
