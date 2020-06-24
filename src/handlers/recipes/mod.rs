use crate::db_connection::PostgresConnection;
use crate::models::recipe::{NewRecipe, Recipe, ThinRecipe};
use crate::models::transport::{ApiResponse};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/api/recipe?<limit>&<offset>&<include_archived>")]
pub fn get_recipes(limit: Option<i64>, offset: Option<i64>, include_archived: Option<bool>, connection: PostgresConnection) -> ApiResponse<Vec<ThinRecipe>> {
    let result = ThinRecipe::find_many(limit, offset, include_archived, &connection);

    match result {
        Ok(recipes) => ApiResponse {
            json: Json(Some(recipes)),
            status: Status::Ok,
        },
        Err(error) => {
            println!("Unable to query for recipes :: limit: {}, offset: {}, include_archived: {} :: {}", limit.unwrap_or(0), offset.unwrap_or(0), include_archived.unwrap_or(false), error);
            ApiResponse {
                json: Json(None.into()),
                status: Status::Accepted
            }
        }
    }
}

#[get("/api/recipe/<id>")]
pub fn get_single_recipe(id: i32, connection: PostgresConnection) -> ApiResponse<ThinRecipe> {
    let result = ThinRecipe::find(&id, &connection);

    match result {
        Ok(recipe) => ApiResponse {
            json: Json(Some(recipe)),
            status: Status::Ok,
        },
        Err(error) => {
            println!(
                "No result returned from get_single_recipe: {}",
                error
            );
            ApiResponse {
                json: Json(None.into()),
                status: Status::BadRequest
            }
        }
    }
}

#[post("/api/recipe", format = "json", data = "<raw_recipe>")]
pub fn create_single_recipe(
    raw_recipe: Json<NewRecipe>,
    connection: PostgresConnection,
) -> ApiResponse<ThinRecipe> {
    let recipe = raw_recipe.into_inner();
    let result = Recipe::create(&recipe, &connection);

    match result {
        Ok(recipe) => ApiResponse {
            json: Json(Some(recipe)),
            status: Status::Ok,
        },
        Err(error) => {
            println!(
                "Unable to create new recipe {}: {}",
                recipe.name,
                error
            );
            ApiResponse {
                json: Json(None.into()),
                status: Status::BadRequest,
            }
        }
    }
}

#[put("/api/recipe/<id>", format = "json", data = "<raw_recipe>")]
pub fn update_single_recipe(
    id: i32,
    raw_recipe: Json<NewRecipe>,
    connection: PostgresConnection,
) -> ApiResponse<ThinRecipe> {
    let recipe_data = raw_recipe.into_inner();
    let result = Recipe::update(id, &recipe_data, &connection);

    match result {
        Ok(recipe) => ApiResponse {
            json: Json(Some(recipe)),
            status: Status::Ok,
        },
        Err(error) => {
            println!(
                "Unabled to update recipe {}: {}",
                recipe_data.name,
                error
            );
            ApiResponse {
                json: Json(None.into()),
                status: Status::BadRequest,
            }
        }
    }
}

#[delete("/api/recipe/<id>")]
pub fn archive_single_recipe(
    id: i32,
    connection: PostgresConnection,
) -> ApiResponse<ThinRecipe> {
    let result = Recipe::archive(id, &connection);

    match result {
        Ok(recipe) => ApiResponse {
            json: Json(Some(recipe)),
            status: Status::Ok
        },
        Err(error) => {
            println!("Unable to archive recipe {}: {}", id, error);
            ApiResponse {
                json: Json(None.into()),
                status: Status::BadRequest,
            }
        }
    }
}
