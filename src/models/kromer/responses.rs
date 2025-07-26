use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ApiResponse<'a, T: Serialize> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<ResponseMeta>,

    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError<'a>>,

    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub message: Option<&'a str>,
}

/// A struct with nothing, used as a default placeholder
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct None {}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ResponseMeta {
    pub limit: i32,
    pub total: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ApiError<'a> {
    pub code: &'a str,
    pub message: &'a str,
    pub details: &'a [ErrorDetail<'a>],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ErrorDetail<'a> {
    pub field: &'a str,
    pub message: &'a str,
}

impl<'a, T: Serialize> Default for ApiResponse<'a, T> {
    fn default() -> Self {
        Self {
            data: None,
            meta: None,
            error: None,
            message: None,
        }
    }
}
