use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub title: i32,
    pub description: i32,
}
