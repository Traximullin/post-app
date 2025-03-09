use std::env;

use sqlx::{PgPool, Pool, Postgres};

pub async fn init_db() -> Pool<Postgres> {
    let url = env::var("DATABASE_URL").expect("Не установлено значение переменной DATABASE_URL");

    let db_pool = PgPool::connect(&url)
        .await
        .expect("Не удалось подключиться к БД.");

    return db_pool;
}
