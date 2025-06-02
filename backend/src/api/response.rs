use serde_json::json;

pub type ApiResult = Result<ApiResponse, ApiResponse>;

#[derive(Responder)]
pub(crate) enum ApiResponse {
    #[response(status = 204)]
    NoContent(()),

    #[response(status = 200)]
    Ok(String),

    #[response(status = 400)]
    Err(String),
}
impl ApiResponse {
    fn message_string(message: &str) -> String {
        let value = json!({"message": message});
        serde_json::to_string(&value).unwrap()
    }
    fn key_value(key: &str, value: &str) -> String {
        let value = json!({key: value});
        serde_json::to_string(&value).unwrap()
    }
    pub fn ok_key_value(key: &str, value: &str) -> ApiResponse {
        Self::Ok(Self::key_value(key, value))
    }
    pub fn _ok_message(body: &str) -> ApiResponse {
        Self::Ok(Self::message_string(body))
    }
    pub fn err_message(body: &str) -> ApiResponse {
        Self::Err(Self::message_string(body))
    }
}

impl Into<ApiResult> for ApiResponse {
    fn into(self) -> ApiResult {
        Ok(self)
    }
}
