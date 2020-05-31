use crate::db_connection::PostgresConnection;
use crate::models::recipe::{NewRecipe, Recipe, ThinRecipe};
use crate::models::{ApiResponse, Response};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/api/recipe/<id>")]
pub fn get_single_recipe(id: i32, connection: PostgresConnection) -> Json<Response<ThinRecipe>> {
    let result = ThinRecipe::find(&id, &connection);

    if !result.is_ok() {
        println!(
            "No result returned from get_single_recipe: {}",
            result.err().unwrap()
        );
        return Json(None.into());
    }

    Json(result.ok().into())
}

#[post("/api/recipe", format = "json", data = "<raw_recipe>")]
pub fn create_single_recipe(
    raw_recipe: Json<NewRecipe>,
    connection: PostgresConnection,
) -> Json<Response<ThinRecipe>> {
    let recipe = raw_recipe.0;
    let result = Recipe::create(&recipe, &connection);

    if !result.is_ok() {
        println!(
            "Unable to create new recipe {}: {}",
            recipe.name,
            result.err().unwrap()
        );
        return Json(None.into());
    }

    Json(result.ok().into())
}

#[put("/api/recipe/<id>", format = "json", data = "<raw_recipe>")]
pub fn update_single_recipe(
    id: i32,
    raw_recipe: Json<NewRecipe>,
    connection: PostgresConnection,
) -> ApiResponse<ThinRecipe> {
    let recipe_data = raw_recipe.0;
    let result = Recipe::update(id, &recipe_data, &connection);

    if !result.is_ok() {
        println!(
            "Unabled to update recipe {}: {}",
            recipe_data.name,
            result.err().unwrap()
        );
        return ApiResponse {
            json: Json(None.into()),
            status: Status::BadRequest,
        };
    }

    ApiResponse {
        json: Json(result.ok()),
        status: Status::Accepted,
    }
}
