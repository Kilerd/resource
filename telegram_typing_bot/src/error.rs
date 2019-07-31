use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub ok: bool,
    pub error_code: Option<i32>,
    pub description: Option<String>,
    pub result: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub error_code: i32,
    pub description: String,
}

pub type BotResult<T> = Result<T, ApiError>;

impl<T> ApiResult<T> {
    fn as_result(self) -> BotResult<T> {
        if self.ok {
            Ok(self.result.unwrap())
        } else {
            Err(ApiError {
                error_code: self.error_code.expect("undetected condition"),
                description: self.description.expect("undetected condition"),
            })
        }
    }
}
