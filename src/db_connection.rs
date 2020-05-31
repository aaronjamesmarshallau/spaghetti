use diesel::pg::PgConnection;
use diesel::r2d2;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

fn get_database_url() -> String {
    dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn init_pool() -> Pool {
    let manager = r2d2::ConnectionManager::new(get_database_url());
    Pool::new(manager).expect("Database pool creation failed")
}

pub struct PostgresConnection(pub r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for PostgresConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<PostgresConnection, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(PostgresConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for PostgresConnection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
