-- Represents a single ingredient type
CREATE TABLE ingredient(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    image_url VARCHAR NOT NULL
);

-- Represents a recipe
CREATE TABLE recipe(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    image_url VARCHAR NOT NULL
);

-- Represents the relationship between a recipe and an ingredient
CREATE TABLE recipe_ingredient(
    id SERIAL PRIMARY KEY NOT NULL,
    ingredient_id INT REFERENCES ingredient (id),
    recipe_id INT REFERENCES recipe(id),
    quantity DECIMAL NOT NULL,
    unit SMALLINT NOT NULL
);
