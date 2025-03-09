use crate::state::AppState;
use axum::Router;
use posts::PostController;

mod posts;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(PostController::routes())
        .with_state(state)
}
