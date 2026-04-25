use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AiQueryRequest {
    pub query: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AiAction {
    pub tool: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AiQueryResponse {
    pub answer: String,
    pub actions_taken: Vec<AiAction>,
    pub suggested_commands: Vec<String>,
}
