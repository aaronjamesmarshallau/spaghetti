use rocket_contrib::json::Json;
use crate::models::recipe::ThinRecipe;
use crate::db_connection::PostgresConnection;
use crate::models::Response;

#[get("/api/recipe/<id>")]
pub fn get_single_recipe(id: i32, connection: PostgresConnection) -> Json<Response<ThinRecipe>> {
    let result = ThinRecipe::find(&id, &connection);

    if !result.is_ok() {
        println!("No result returned from get_single_recipe...");
    }

    Json(result.ok().into())
}