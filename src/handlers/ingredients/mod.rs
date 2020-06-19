use crate::db_connection::PostgresConnection;
use crate::models::recipe::ingredient::ExpandedRecipeIngredient;
use crate::models::recipe::ingredient::{
    Ingredient, NewIngredient, NewRecipeIngredient, RecipeIngredient,
};
use crate::models::ApiResponse;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/api/recipe/<recipe_id>/ingredients")]
pub fn get_recipe_ingredients(
    recipe_id: i32,
    connection: PostgresConnection,
) -> ApiResponse<Vec<ExpandedRecipeIngredient>> {
    let result = RecipeIngredient::find_by_recipe(recipe_id, &connection);

    match result {
        Ok(recipes) => ApiResponse {
            json: Json(Some(recipes.into_iter().map(Into::into).collect())),
            status: Status::Accepted,
        },
        Err(err) => {
            println!(
                "Unable to retrieve recipe ingredients for {}: {}",
                recipe_id, err
            );

            ApiResponse {
                json: Json(None.into()),
                status: Status::BadRequest,
            }
        }
    }
}

#[post(
    "/api/recipe/<recipe_id>/ingredients",
    format = "json",
    data = "<new_ingredient>"
)]
pub fn create_recipe_ingredient(
    recipe_id: i32,
    new_ingredient: Json<NewRecipeIngredient>,
    connection: PostgresConnection,
) -> ApiResponse<RecipeIngredient> {
    let result = RecipeIngredient::create(recipe_id, &new_ingredient, &connection);

    match result {
        Ok(recipe_ingredient) => ApiResponse {
            json: Json(Some(recipe_ingredient)),
            status: Status::Accepted,
        },
        Err(error) => {
            println!(
                "Failed to create recipe ingredient {} for recipe {}: {}",
                new_ingredient.ingredient_id, recipe_id, error
            );
            ApiResponse {
                json: Json(None.into()),
                status: Status::BadRequest,
            }
        }
    }
}

#[post(
    "/api/recipe/<recipe_id>/ingredients/<recipe_ingredient_id>",
    format = "json",
    data = "<new_ingredient>"
)]
pub fn update_recipe_ingredient(
    recipe_id: i32,
    recipe_ingredient_id: i32,
    new_ingredient: Json<NewRecipeIngredient>,
    connection: PostgresConnection,
) -> ApiResponse<RecipeIngredient> {
    let result = RecipeIngredient::update(
        recipe_id,
        recipe_ingredient_id,
        &new_ingredient,
        &connection,
    );

    match result {
        Ok(recipe_ingredient) => ApiResponse {
            json: Json(Some(recipe_ingredient)),
            status: Status::Accepted,
        },
        Err(error) => {
            println!(
                "Failed to create recipe ingredient {} for recipe {}: {}",
                new_ingredient.ingredient_id, recipe_id, error
            );
            ApiResponse {
                json: Json(None.into()),
                status: Status::BadRequest,
            }
        }
    }
}

#[post("/api/ingredients", format = "json", data = "<new_ingredient>")]
pub fn create_ingredient(
    new_ingredient: Json<NewIngredient>,
    connection: PostgresConnection,
) -> ApiResponse<Ingredient> {
    let ingredient = new_ingredient.0;
    let result = Ingredient::create(&ingredient, &connection);

    match result {
        Ok(ingredient) => ApiResponse {
            json: Json(Some(ingredient)),
            status: Status::Accepted,
        },
        Err(error) => {
            println!(
                "Failed to created ingredient {}: {}",
                ingredient.name, error
            );
            ApiResponse {
                json: Json(None.into()),
                status: Status::BadRequest,
            }
        }
    }
}

#[put("/api/ingredients/<id>", format = "json", data = "<ingredient_data>")]
pub fn update_ingredient(
    id: i32,
    ingredient_data: Json<NewIngredient>,
    connection: PostgresConnection,
) -> ApiResponse<Ingredient> {
    let ingredient = ingredient_data.0;
    let result = Ingredient::update(id, &ingredient, &connection);

    match result {
        Ok(ingredient) => ApiResponse {
            json: Json(Some(ingredient)),
            status: Status::Accepted,
        },
        Err(error) => {
            println!("Failed to update ingredient {}: {}", id, error);
            ApiResponse {
                json: Json(None.into()),
                status: Status::BadRequest,
            }
        }
    }
}
