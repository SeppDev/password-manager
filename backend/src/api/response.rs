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
    fn message_string(body: impl ToString) -> String {
        let message = body.to_string();
        let value = json!({"message": message});
        serde_json::to_string(&value).unwrap()
    }
    fn key_value(key: impl ToString, value: impl ToString) -> String {
        let key = key.to_string();
        let value = value.to_string();
        let value = json!({key: value});
        serde_json::to_string(&value).unwrap()
    }
    pub fn ok_key_value(key: impl ToString, value: impl ToString) -> ApiResponse {
        Self::Ok(Self::key_value(key, value))
    }
    pub fn err_key_value(key: impl ToString, value: impl ToString) -> ApiResponse {
        Self::Err(Self::key_value(key, value))
    }
    pub fn ok_message(body: impl ToString) -> ApiResponse {
        Self::Ok(Self::message_string(body))
    }
    pub fn err_message(body: impl ToString) -> ApiResponse {
        Self::Err(Self::message_string(body))
    }
}

impl Into<ApiResult> for ApiResponse {
    fn into(self) -> ApiResult {
        Ok(self)
    }
}