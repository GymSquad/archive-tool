use crate::prelude::*;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

type DbConnectionPool = Pool<Postgres>;

#[derive(Debug, PartialEq, Eq, sqlx::FromRow, Clone)]
pub struct Website {
    pub id: String,
    pub url: String,
    pub is_valid: bool,
}

pub struct Database {
    pool: DbConnectionPool,
}

impl Database {
    pub async fn connect(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn get_all_urls(&self) -> Result<Vec<Website>> {
        sqlx::query_as::<_, Website>(r#"SELECT id, url, "isValid" as is_valid FROM "Website""#)
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }

    async fn update_website(&self, website: &Website) -> Result<()> {
        sqlx::query(r#"UPDATE "Website" SET "isValid" = $1 WHERE id = $2"#)
            .bind(website.is_valid)
            .bind(&website.id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_validity(&self, website: &Website, is_valid: bool) -> Result<()> {
        if website.is_valid == is_valid {
            return Ok(());
        }

        let mut website = website.clone();
        website.is_valid = is_valid;

        self.update_website(&website).await
    }
}
