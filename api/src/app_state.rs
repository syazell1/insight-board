use sqlx::PgPool;

use crate::config::JwtSettings;

pub struct AppState {
    pub pool : PgPool,
    pub jwt_settings : JwtSettings,
    pub client_url : String
}