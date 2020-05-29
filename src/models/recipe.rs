use diesel::{Queryable, PgConnection};
use serde::{Serialize, Deserialize};
use crate::schema::{recipe};

#[derive(Queryable, Serialize, Deserialize)]
pub struct ThinRecipe {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl ThinRecipe {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<ThinRecipe, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        recipe::table.find(id).select((recipe::id, recipe::name, recipe::description)).first(connection)
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
}

impl From<Recipe> for ThinRecipe {
    fn from(rcp: Recipe) -> ThinRecipe {
        ThinRecipe {
            id: rcp.id,
            name: rcp.name,
            description: rcp.description,
        }
    }
}

impl Recipe {
    pub fn create(rcp: &NewRecipe, connection: &PgConnection) -> Result<ThinRecipe, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(recipe::table)
            .values(rcp)
            .get_result::<Recipe>(connection)
            .map(Into::into)
    }
}