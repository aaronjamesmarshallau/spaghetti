-- Add `tags` table for 'recipes'
CREATE TABLE recipe_tag(
    recipe_id INT REFERENCES recipe(id),
    name VARCHAR NOT NULL,
    PRIMARY KEY (recipe_id, name)
);

-- Add `tags` table for 'ingredients'
CREATE TABLE ingredient_tag(
    ingredient_id INT REFERENCES ingredient(id),
    name VARCHAR NOT NULL,
    PRIMARY KEY (ingredient_id, name)
);
