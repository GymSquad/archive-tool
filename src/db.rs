use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type DbConnectionPool = Pool<Postgres>;

#[derive(Debug, PartialEq, Eq, sqlx::FromRow)]
pub struct Website {
    pub id: String,
    pub url: String,
    pub is_valid: bool,
}

pub async fn create_connection_pool(database_url: &str) -> anyhow::Result<DbConnectionPool> {
    PgPoolOptions::new()
        .max_connections(2)
        .connect(database_url)
        .await
        .map_err(Into::into)
}

pub async fn get_all_urls(pool: &DbConnectionPool) -> anyhow::Result<Vec<Website>> {
    sqlx::query_as::<_, Website>(r#"SELECT id, url, "isValid" as is_valid FROM "Website""#)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
}

pub async fn update_website(pool: &DbConnectionPool, website: &Website) -> anyhow::Result<()> {
    sqlx::query(r#"UPDATE "Website" SET "isValid" = $1 WHERE id = $2"#)
        .bind(website.is_valid)
        .bind(&website.id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(Into::into)
}
