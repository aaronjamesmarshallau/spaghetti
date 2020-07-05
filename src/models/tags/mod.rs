use crate::schema::recipe_tag;
use crate::schema::ingredient_tag;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, ExpressionMethods};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "recipe_tag"]
pub struct RecipeTag {
    pub recipe_id: i32,
    pub name: String,
}

impl RecipeTag {
    pub fn find_by_recipe_id(
        id: i32,
        connection: &PgConnection,
    ) -> Result<Vec<String>, diesel::result::Error> {
        use crate::schema::recipe_tag::dsl::*;

        recipe_tag
            .filter(recipe_id.eq(id))
            .select(name)
            .get_results(connection)
    }

    pub fn save_recipe_tag(
        tag_data: &RecipeTag,
        connection: &PgConnection,
    ) -> Result<RecipeTag, diesel::result::Error> {
        use crate::schema::recipe_tag::dsl::*;

        diesel::insert_into(recipe_tag)
            .values((
                recipe_id.eq(tag_data.recipe_id),
                name.eq(&tag_data.name)
            ))
            .get_result(connection)
    }

    pub fn remove_recipe_tag(
        tag_data: &RecipeTag,
        connection: &PgConnection,
    ) -> Result<RecipeTag, diesel::result::Error> {
        use crate::schema::recipe_tag::dsl::*;

        diesel::delete(recipe_tag)
            .filter(recipe_id.eq(tag_data.recipe_id))
            .filter(name.eq(&tag_data.name))
            .returning((recipe_id, name))
            .get_result::<RecipeTag>(connection)
    }
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "ingredient_tag"]
pub struct IngredientTag {
    pub ingredient_id: i32,
    pub name: String,
}

impl IngredientTag {
    pub fn find_by_ingredient_id(
        id: i32,
        connection: &PgConnection,
    ) -> Result<Vec<String>, diesel::result::Error> {
        use crate::schema::ingredient_tag::dsl::*;

        ingredient_tag
            .filter(ingredient_id.eq(id))
            .select(name)
            .get_results(connection)
    }

    pub fn save_ingredient_tag(
        tag_data: &IngredientTag,
        connection: &PgConnection,
    ) -> Result<IngredientTag, diesel::result::Error> {
        use crate::schema::ingredient_tag::dsl::*;

        diesel::insert_into(ingredient_tag)
            .values((
                ingredient_id.eq(tag_data.ingredient_id),
                name.eq(&tag_data.name),
            ))
            .get_result(connection)
    }

    pub fn remove_ingredient_tag(
        tag_data: &IngredientTag,
        connection: &PgConnection,
    ) -> Result<IngredientTag, diesel::result::Error> {
        use crate::schema::ingredient_tag::dsl::*;

        diesel::delete(ingredient_tag)
            .filter(ingredient_id.eq(tag_data.ingredient_id))
            .filter(name.eq(&tag_data.name))
            .returning((ingredient_id, name))
            .get_result::<IngredientTag>(connection)
    }
}
