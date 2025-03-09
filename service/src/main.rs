use dotenv::dotenv;
use state::AppState;
use tokio::net::TcpListener;

mod db;
mod dto;
mod handlers;
mod models;
mod repositories;
mod state;
mod use_cases;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db::init_db().await;

    let state = AppState::new(pool);

    let app = handlers::create_router(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
