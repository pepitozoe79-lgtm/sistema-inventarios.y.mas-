use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Serialize, ToSchema)]
pub struct Meta {
    pub total: usize,
}

#[derive(Serialize, ToSchema)]
pub struct ApiListResponse<T> {
    pub data: Vec<T>,
    pub meta: Meta,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> ApiListResponse<T> {
    pub fn new(data: Vec<T>) -> Self {
        let total = data.len();
        Self {
            data,
            meta: Meta { total },
        }
    }
}
