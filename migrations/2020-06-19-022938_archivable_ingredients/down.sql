-- Remove `archived` from `recipe`
ALTER TABLE recipe
DROP COLUMN "archived";

-- Remove `archived` from `ingredient`
ALTER TABLE ingredient
DROP COLUMN "archived";

-- Remove `archived` from `recipe_ingredient`
ALTER TABLE recipe_ingredient
DROP COLUMN "archived";
