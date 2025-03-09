use sqlx::{Error, PgPool};

use crate::{dto::posts::CreatePostDTO, models::posts::Post, repositories::posts::PostRepository};

pub struct PostService;

impl PostService {
    pub async fn get_posts(pool: &PgPool) -> Result<Vec<Post>, Error> {
        PostRepository::get_all_posts(pool).await
    }

    pub async fn create_post(pool: &PgPool, new_post: CreatePostDTO) -> Result<Post, Error> {
        PostRepository::create_post(pool, new_post).await
    }

    pub async fn delete_post(pool: &PgPool, id: i32) -> Result<Post, Error> {
        PostRepository::delete_post(pool, id).await
    }
}
