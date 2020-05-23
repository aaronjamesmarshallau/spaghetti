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