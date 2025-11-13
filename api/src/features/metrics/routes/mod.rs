use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    app_state::AppState, features::metrics::routes::get_all_api_metrics::get_all_api_metrics,
};

mod get_all_api_metrics;

pub fn metric_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_all_api_metrics))
}
