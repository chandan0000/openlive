use axum::Router;
use axum::routing::get;

use crate::handler::user_handler;
use crate::utils::app_state::AppState;

pub fn user_create_route() -> Router<AppState> {
    Router::<AppState>::new().route("/profile/{user_id}", get(user_handler::user_profile_get))
}
