use crate::db_connection::PostgresConnection;
use crate::models::tags::{RecipeTag, IngredientTag};
use crate::models::transport::{ApiResponse};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/api/tags/recipe/<id>")]
pub fn get_tags_for_recipe(id: i32, connection: PostgresConnection) -> ApiResponse<Vec<String>> {
    let result = RecipeTag::find_by_recipe_id(id, &connection);

    match result {
        Ok(tag_names) => ApiResponse {
            json: Json(Some(tag_names)),
            status: Status::Ok,
        },
        Err(error) => {
            println!("Unable to query for recipe tags :: recipe_id: {} :: {}", id, error);
            ApiResponse {
                json: Json(None.into()),
                status: Status::Accepted,
            }
        }
    }
}

#[post("/api/tags/recipe", format = "json", data = "<raw_tag>")]
pub fn save_recipe_tag(
    raw_tag: Json<RecipeTag>,
    connection: PostgresConnection,
) -> ApiResponse<RecipeTag> {
    let tag_data = raw_tag.into_inner();
    let result = RecipeTag::save_recipe_tag(&tag_data, &connection);

    match result {
        Ok(tag) => ApiResponse {
            json: Json(Some(tag)),
            status: Status::Ok,
        },
        Err(error) => {
            println!("Unable to save tag for recipe :: recipe_id: {}, name: {} :: {}", tag_data.recipe_id, tag_data.name, error);
            ApiResponse {
                json: Json(None.into()),
                status: Status::Accepted,
            }
        }
    }
}

#[delete("/api/tags/recipe", format = "json", data = "<raw_tag>")]
pub fn remove_recipe_tag(
    raw_tag: Json<RecipeTag>,
    connection: PostgresConnection,
) -> ApiResponse<RecipeTag> {
    let tag_data = raw_tag.into_inner();
    let result = RecipeTag::remove_recipe_tag(&tag_data, &connection);

    match result {
        Ok(tag) => ApiResponse {
            json: Json(Some(tag)),
            status: Status::Ok,
        },
        Err(error) => {
            println!("Unable to remove tag for recipe :: recipe_id: {}, name: {} :: {}", tag_data.recipe_id, tag_data.name, error);
            ApiResponse {
                json: Json(None.into()),
                status: Status::Accepted,
            }
        }
    }
}

#[get("/api/tags/ingredient/<id>")]
pub fn get_tags_for_ingredient(id: i32, connection: PostgresConnection) -> ApiResponse<Vec<String>> {
    let result = IngredientTag::find_by_ingredient_id(id, &connection);

    match result {
        Ok(tag_names) => ApiResponse {
            json: Json(Some(tag_names)),
            status: Status::Ok,
        },
        Err(error) => {
            println!("Unable to query for ingredient tags :: ingredient_id: {} :: {}", id, error);
            ApiResponse {
                json: Json(None.into()),
                status: Status::Accepted,
            }
        }
    }
}

#[post("/api/tags/ingredient", format = "json", data = "<raw_tag>")]
pub fn save_ingredient_tag(
    raw_tag: Json<IngredientTag>,
    connection: PostgresConnection,
) -> ApiResponse<IngredientTag> {
    let tag_data = raw_tag.into_inner();
    let result = IngredientTag::save_ingredient_tag(&tag_data, &connection);

    match result {
        Ok(tag) => ApiResponse {
            json: Json(Some(tag)),
            status: Status::Ok,
        },
        Err(error) => {
            println!("Unable to save tag for ingredient :: ingredient_id: {}, name: {} :: {}", tag_data.ingredient_id, tag_data.name, error);
            ApiResponse {
                json: Json(None.into()),
                status: Status::Accepted,
            }
        }
    }
}

#[delete("/api/tags/ingredient", format = "json", data = "<raw_tag>")]
pub fn remove_ingredient_tag(
    raw_tag: Json<IngredientTag>,
    connection: PostgresConnection,
) -> ApiResponse<IngredientTag> {
    let tag_data = raw_tag.into_inner();
    let result = IngredientTag::remove_ingredient_tag(&tag_data, &connection);

    match result {
        Ok(tag) => ApiResponse {
            json: Json(Some(tag)),
            status: Status::Ok,
        },
        Err(error) => {
            println!("Unable to remove tag for ingredient :: ingredient_id: {}, name: {} :: {}", tag_data.ingredient_id, tag_data.name, error);
            ApiResponse {
                json: Json(None.into()),
                status: Status::Accepted,
            }
        }
    }
}
