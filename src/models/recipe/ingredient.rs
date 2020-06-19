use crate::conversion::units::UnitOfMeasurement;
use crate::schema::ingredient;
use crate::schema::recipe_ingredient;
use bigdecimal::BigDecimal;
use diesel::{PgConnection, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "recipe_ingredient"]
pub struct NewRecipeIngredient {
    pub ingredient_id: i32,
    pub quantity: BigDecimal,
    pub unit: UnitOfMeasurement,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub quantity: BigDecimal,
    pub unit: UnitOfMeasurement,
    pub archived: bool,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct ExpandedRecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub ingredient_name: String,
    pub ingredient_description: String,
    pub ingredient_image: String,
    pub quantity: BigDecimal,
    pub unit: UnitOfMeasurement,
}

impl RecipeIngredient {
    pub fn find_by_recipe(
        rec_id: i32,
        connection: &PgConnection,
    ) -> Result<Vec<ExpandedRecipeIngredient>, diesel::result::Error> {
        use crate::schema::recipe_ingredient::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

        recipe_ingredient
            .inner_join(ingredient::table)
            .filter(recipe_id.eq(rec_id))
            .filter(archived.eq(false))
            .select((
                id,
                recipe_id,
                ingredient::id,
                ingredient::name,
                ingredient::description,
                ingredient::image_url,
                quantity,
                unit,
            ))
            .get_results(connection)
    }

    pub fn create(
        rec_id: i32,
        ingredient_data: &NewRecipeIngredient,
        connection: &PgConnection,
    ) -> Result<RecipeIngredient, diesel::result::Error> {
        use crate::schema::recipe_ingredient::dsl::*;
        use diesel::{ExpressionMethods, RunQueryDsl};

        diesel::insert_into(recipe_ingredient)
            .values((
                ingredient_id.eq(ingredient_data.ingredient_id),
                recipe_id.eq(rec_id),
                quantity.eq(&ingredient_data.quantity),
                unit.eq(&ingredient_data.unit),
            ))
            .get_result(connection)
    }

    pub fn update(
        rec_id: i32,
        rec_ing_id: i32,
        ingredient_data: &NewRecipeIngredient,
        connection: &PgConnection,
    ) -> Result<RecipeIngredient, diesel::result::Error> {
        use crate::schema::recipe_ingredient::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

        diesel::update(recipe_ingredient.filter(id.eq(rec_ing_id)))
            .set((
                ingredient_id.eq(ingredient_data.ingredient_id),
                recipe_id.eq(rec_id),
                quantity.eq(&ingredient_data.quantity),
                unit.eq(&ingredient_data.unit),
            ))
            .get_result(connection)
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "ingredient"]
pub struct NewIngredient {
    pub name: String,
    pub description: String,
    pub image_url: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub archived: bool,
}

impl Ingredient {
    pub fn create(
        new_ingredient: &NewIngredient,
        connection: &PgConnection,
    ) -> Result<Ingredient, diesel::result::Error> {
        use crate::schema::ingredient::dsl::*;
        use diesel::RunQueryDsl;

        diesel::insert_into(ingredient)
            .values(new_ingredient)
            .get_result(connection)
    }

    pub fn update(
        ingredient_id: i32,
        ingredient_data: &NewIngredient,
        connection: &PgConnection,
    ) -> Result<Ingredient, diesel::result::Error> {
        use crate::schema::ingredient::dsl::*;
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

        let ingredient_name = &ingredient_data.name;
        let ingredient_description = &ingredient_data.description;
        let ingredient_image = &ingredient_data.image_url;

        diesel::update(ingredient.filter(id.eq(ingredient_id)))
            .set((
                name.eq(ingredient_name),
                description.eq(ingredient_description),
                image_url.eq(ingredient_image.as_ref().ok_or("").unwrap()),
            ))
            .get_result(connection)
    }
}
