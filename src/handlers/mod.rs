use rocket_contrib::json::Json;
use crate::models::recipe::{ThinRecipe, Recipe, NewRecipe};
use crate::db_connection::PostgresConnection;
use crate::models::Response;

#[get("/api/recipe/<id>")]
pub fn get_single_recipe(id: i32, connection: PostgresConnection) -> Json<Response<ThinRecipe>> {
    let result = ThinRecipe::find(&id, &connection);

    if !result.is_ok() {
        println!("No result returned from get_single_recipe: {}", result.err().unwrap());
        return Json(None.into());
    }

    Json(result.ok().into())
}

#[post("/api/recipe", format = "application/json", data = "<raw_recipe>")]
pub fn create_single_recipe(raw_recipe: Json<NewRecipe>, connection: PostgresConnection) -> Json<Response<ThinRecipe>> {
    let recipe = raw_recipe.0;
    let result = Recipe::create(&recipe, &connection);

    if !result.is_ok() {
        println!("Unable to create new recipe {}: {}", recipe.name, result.err().unwrap());
        return Json(None.into());
    }

    Json(result.ok().into())
}