-- Add `tags` table for 'recipes'
CREATE TABLE recipe_tags(
    recipe_id INT REFERENCES recipe(id),
    name VARCHAR NOT NULL
    PRIMARY KEY (recipe_id, name);
);

-- Add `tags` table for 'ingredients'
CREATE TABLE recipe_tags(
    ingredient_id INT REFERENCES ingredients(id),
    name VARCHAR NOT NULL
    PRIMARY KEY (ingredient_id, name);
);
