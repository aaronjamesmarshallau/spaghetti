use crate::db_connection::PostgresConnection;
use crate::models::recipe::ingredient::ExpandedRecipeIngredient;
use crate::models::recipe::ingredient::{
    Ingredient, NewIngredient, NewRecipeIngredient, RecipeIngredient,
};
use crate::models::transport::{ApiResponse, OperationResponse};
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
            status: Status::Ok,
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
            status: Status::Ok,
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

#[put(
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
        recipe_ingredient_id,
        &new_ingredient,
        &connection,
    );

    match result {
        Ok(recipe_ingredient) => ApiResponse {
            json: Json(Some(recipe_ingredient)),
            status: Status::Ok,
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

#[delete("/api/recipe/<recipe_id>/ingredients/<recipe_ingredient_id>")]
pub fn archive_recipe_ingredient(
    recipe_id: i32, 
    recipe_ingredient_id: i32, 
    connection: PostgresConnection
) -> ApiResponse<OperationResponse> {
    let result = RecipeIngredient::archive(recipe_ingredient_id, &connection);

    match result {
        Ok(_) => {
            let op_res = OperationResponse {
                success: true,
                message: format!("The ingredient {} for recipe {} was archived successfully.", recipe_ingredient_id, recipe_id),
            };

            ApiResponse {
                json: Json(Some(op_res)),
                status: Status::Ok
            }
        },
        Err(error) => {
            let message = format!("The ingredient {} for recipe {} could not be archived", recipe_ingredient_id, recipe_id);
            
            println!("{}: {}", message, error);

            let op_res = OperationResponse {
                success: false,
                message,
            };

            ApiResponse {
                json: Json(Some(op_res)),
                status: Status::BadRequest
            }
        }
    }
}

#[post("/api/ingredients", format = "json", data = "<new_ingredient>")]
pub fn create_ingredient(
    new_ingredient: Json<NewIngredient>,
    connection: PostgresConnection,
) -> ApiResponse<Ingredient> {
    let ingredient = new_ingredient.into_inner();
    let result = Ingredient::create(&ingredient, &connection);

    match result {
        Ok(ingredient) => ApiResponse {
            json: Json(Some(ingredient)),
            status: Status::Ok,
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

#[get("/api/ingredients/<id>")]
pub fn get_single_ingredient(
    id: i32,
    connection: PostgresConnection
) -> ApiResponse<Ingredient> {
    let result = Ingredient::find_one(id, &connection);

    match result {
        Ok(ingredient) => ApiResponse {
            json: Json(Some(ingredient)),
            status: Status::Ok,
        },
        Err(error) => {
            println!("Unable to retrieve ingredient with id {}: {}", id, error);

            ApiResponse {
                json: Json(None.into()),
                status: Status::BadRequest
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
    let ingredient = ingredient_data.into_inner();
    let result = Ingredient::update(id, &ingredient, &connection);

    match result {
        Ok(ingredient) => ApiResponse {
            json: Json(Some(ingredient)),
            status: Status::Ok,
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

#[delete("/api/ingredients/<id>")]
pub fn archive_ingredient(
    id: i32,
    connection: PostgresConnection
) -> ApiResponse<Ingredient> {
    let result = Ingredient::archive(id, &connection);

    match result {
        Ok(ingredient) => ApiResponse {
            json: Json(Some(ingredient)),
            status: Status::Ok,
        },
        Err(error) => {
            println!("Unable to archive ingredient {}: {}", id, error);
            ApiResponse {
                json: Json(None.into()),
                status: Status::BadRequest
            }
        }
    }
}
