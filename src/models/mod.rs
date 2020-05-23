pub mod recipe;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i8>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}

impl <T> From<Option<T>> for Response<T> {
    fn from(item: Option<T>) -> Response<T> {
        Response::<T> {
            status: None,
            message: None,
            data: item,
        }
    }
}

impl <T> From<T> for Response<T> {
    fn from(item: T) -> Response<T> {
        Response::<T> {
            status: None,
            message: None,
            data: Some(item),
        }
    }
}