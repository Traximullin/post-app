use sqlx::{query_as, Error, PgPool};

use crate::{dto::posts::CreatePostDTO, models::posts::Post};

pub struct PostRepository;

impl PostRepository {
    pub async fn get_all_posts(pool: &PgPool) -> Result<Vec<Post>, Error> {
        let sql = "SELECT * FROM posts";
        let posts = query_as::<_, Post>(sql).fetch_all(pool).await?;

        Ok(posts)
    }

    pub async fn create_post(pool: &PgPool, new_post: CreatePostDTO) -> Result<Post, Error> {
        let sql = r#"
            INSERT INTO todos (title, description)
            VALUES ($1, $2)_at
            RETURNING *
        "#;

        let post = query_as::<_, Post>(sql)
            .bind(new_post.title)
            .bind(new_post.description)
            .fetch_one(pool)
            .await?;

        Ok(post)
    }

    pub async fn delete_post(pool: &PgPool, id: i32) -> Result<Post, Error> {
        let sql = "DELETE FROM posts WHERE id = $1 RETURNING *";

        let post = query_as::<_, Post>(sql).bind(id).fetch_one(pool).await?;

        Ok(post)
    }
}
