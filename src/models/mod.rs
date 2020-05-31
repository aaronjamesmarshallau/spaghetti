pub mod recipe;

use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::Request;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> From<Option<T>> for Response<T> {
    fn from(item: Option<T>) -> Response<T> {
        Response::<T> {
            status: None,
            message: None,
            data: item,
        }
    }
}

impl<T> From<T> for Response<T> {
    fn from(item: T) -> Response<T> {
        Response::<T> {
            status: None,
            message: None,
            data: Some(item),
        }
    }
}

pub struct ApiResponse<T> {
    pub json: Json<Option<T>>,
    pub status: Status,
}

impl<'r, T: Serialize> Responder<'r> for ApiResponse<T> {
    fn respond_to(self, req: &Request) -> rocket::response::Result<'r> {
        rocket::response::Response::build_from(self.json.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
