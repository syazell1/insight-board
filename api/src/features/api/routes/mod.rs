use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{
    app_state::AppState,
    features::api::routes::{
        add_api::add_api_endpoint, delete_api::delete_api_endpoint,
        get_all_users_api::get_all_users_api, update_api::update_api_endpoint,
    },
};

mod add_api;
mod delete_api;
mod get_all_users_api;
mod update_api;

pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_users_api))
        .route("/", post(add_api_endpoint))
        .route("/{id}", delete(delete_api_endpoint))
        .route("/{id}", patch(update_api_endpoint))
}
