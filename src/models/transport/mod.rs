use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::Request;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OperationResponse {
    pub success: bool,
    pub message: String,
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
