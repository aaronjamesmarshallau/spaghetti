use crate::schema::recipe;
use crate::util::clamped::Clamped;
use diesel::{PgConnection, Queryable, QueryDsl, RunQueryDsl, ExpressionMethods};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct ThinRecipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub archived: bool,
}

const MAX_LIMIT: i64 = 200;
const DEFAULT_OFFSET: i64 = 0;
const DEFAULT_LIMIT: i64 = 25;
const DEFAULT_INCLUDE_ARCHIVED: bool = false;

impl ThinRecipe {
    pub fn find_many(offset: Option<i64>, limit: Option<i64>, include_archived: Option<bool>, connection: &PgConnection) -> Result<Vec<ThinRecipe>, diesel::result::Error> {
        use crate::schema::recipe::dsl::*;

        let offset_val = offset.unwrap_or(DEFAULT_OFFSET).clamp_lower(0);
        // annoying syntax because clamped is implemented as an unstable feature
        // https://doc.rust-lang.org/stable/rust-by-example/trait/disambiguating.html
        let limit_val = <i64 as Clamped>::clamp(&limit.unwrap_or(DEFAULT_LIMIT), 0, MAX_LIMIT); 
        let include_archived_val = include_archived.unwrap_or(DEFAULT_INCLUDE_ARCHIVED);

        let mut statement = recipe
            .select((
                id,
                name,
                description,
                image_url,
                archived
            ))
            .offset(offset_val.into())
            .limit(limit_val.into())
            .into_boxed();

        if !include_archived_val {
            statement = statement.filter(archived.eq(false))
        };

        statement.get_results(connection)
    }

    pub fn find(recipe_id: &i32, connection: &PgConnection) -> Result<ThinRecipe, diesel::result::Error> {
        use crate::schema::recipe::dsl::*;

        recipe
            .find(recipe_id)
            .select((
                id,
                name,
                description,
                image_url,
                archived,
            ))
            .first(connection)
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "recipe"]
pub struct NewRecipe {
    pub name: String,
    pub description: String,
    pub image_url: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub archived: bool,
}

impl From<Recipe> for ThinRecipe {
    fn from(rcp: Recipe) -> ThinRecipe {
        ThinRecipe {
            id: rcp.id,
            name: rcp.name,
            description: rcp.description,
            image_url: rcp.image_url,
            archived: rcp.archived,
        }
    }
}

impl Recipe {
    pub fn create(
        rcp: &NewRecipe,
        connection: &PgConnection,
    ) -> Result<ThinRecipe, diesel::result::Error> {
        use crate::schema::recipe::dsl::*;

        diesel::insert_into(recipe)
            .values(rcp)
            .get_result::<Recipe>(connection)
            .map(Into::into)
    }

    pub fn update(
        recipe_id: i32,
        recipe_data: &NewRecipe,
        connection: &PgConnection,
    ) -> Result<ThinRecipe, diesel::result::Error> {
        use crate::schema::recipe::dsl::*;

        let recipe_name = &recipe_data.name;
        let recipe_description = &recipe_data.description;
        let recipe_image = &recipe_data.image_url;

        diesel::update(recipe.filter(id.eq(recipe_id)))
            .set((
                name.eq(recipe_name),
                description.eq(recipe_description),
                image_url.eq(recipe_image),
            ))
            .get_result(connection)
    }

    pub fn archive(
        recipe_id: i32,
        connection: &PgConnection
    ) -> Result<ThinRecipe, diesel::result::Error> {
        use crate::schema::recipe::dsl::*;

        diesel::update(recipe.filter(id.eq(recipe_id)))
            .set(
                archived.eq(true)
            )
            .get_result(connection)
    }
}

pub mod ingredient;
