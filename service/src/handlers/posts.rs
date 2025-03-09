use std::{convert::Infallible, time::Duration};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive},
        IntoResponse, Sse,
    },
    routing::{delete, get},
    Json, Router,
};
use tokio::time::interval;
use tokio_stream::{Stream, StreamExt};

use crate::{dto::posts::CreatePostDTO, state::AppState, use_cases::posts::PostService};

pub struct PostController;

impl PostController {
    pub fn routes() -> Router<AppState> {
        Router::new()
            .route("/posts", get(Self::get_all).post(Self::create))
            .route("/posts/{id}", delete(Self::delete))
            .route("/events-posts", get(Self::sse_get_all))
    }

    async fn get_all(State(state): State<AppState>) -> impl IntoResponse {
        match PostService::get_posts(&state.db).await {
            Ok(posts) => (StatusCode::OK, Json(posts)).into_response(),
            Err(_) => (StatusCode::BAD_REQUEST, "Failed to fetch posts").into_response(),
        }
    }

    async fn create(
        State(state): State<AppState>,
        Json(payload): Json<CreatePostDTO>,
    ) -> impl IntoResponse {
        match PostService::create_post(&state.db, payload).await {
            Ok(post) => (StatusCode::CREATED, Json(post)).into_response(),
            Err(_) => (StatusCode::UNPROCESSABLE_ENTITY, "Failed to create post").into_response(),
        }
    }

    async fn delete(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
        match PostService::delete_post(&state.db, id).await {
            Ok(_) => (StatusCode::NO_CONTENT).into_response(),
            Err(_) => (StatusCode::NOT_FOUND, "Post not found").into_response(),
        }
    }

    async fn sse_get_all(
        State(state): State<AppState>,
    ) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
        let interval = interval(Duration::from_secs(2));

        let stream = tokio_stream::wrappers::IntervalStream::new(interval).then(move |_| {
            let state = state.clone();

            async move {
                match PostService::get_posts(&state.db).await {
                    Ok(posts) => Ok(Event::default().json_data(posts).unwrap()),
                    Err(_) => Ok(Event::default().data("Failed to fetch posts")),
                }
            }
        });

        Sse::new(stream).keep_alive(KeepAlive::default())
    }
}
