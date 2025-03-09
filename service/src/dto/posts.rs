use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreatePostDTO {
    pub title: i32,
    pub description: i32,
}
