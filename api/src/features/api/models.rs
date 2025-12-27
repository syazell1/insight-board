use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ApiFormData {
    pub name: String,
    pub url: String,
    pub description : Option<String>,
    pub interval_secs: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct ApiEndpointData {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub interval_seconds: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}
