use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{types::Uuid, Pool, Postgres};

// Player model to allow for reading from the database directly into a struct.
// Conversion between Rust and Postgres types from
// https://docs.rs/sqlx-core/0.6.0/sqlx_core/postgres/types/index.html.
#[derive(Debug, Serialize)]
pub struct Player {
    id: Uuid,
    name: String,
    score: i32,
    created_at: NaiveDateTime,
}

// Implement a couple methods on the Player struct for CRUD operations.
impl Player {
    // Create a new player in the database with a name & score and return the new player.
    pub async fn new(
        pool: &Pool<Postgres>,
        name: String,
        score: i32,
    ) -> Result<Player, sqlx::Error> {
        sqlx::query_as!(
            Player,
            r#"
                INSERT INTO players (name, score)
                VALUES ($1, $2)
                RETURNING id, name, score, created_at
            "#,
            name,
            score
        )
        .fetch_one(pool)
        .await
    }
    // Get all players from the database and return them in a Vec.
    pub async fn get_all(pool: &Pool<Postgres>) -> Result<Vec<Player>, sqlx::Error> {
        sqlx::query_as!(Player, "SELECT * FROM players")
            .fetch_all(pool)
            .await
    }
    // Update a player in the database with a new name & score and return the updated player.
    pub async fn update(
        pool: &Pool<Postgres>,
        id: Uuid,
        name: String,
        score: i32,
    ) -> Result<Player, sqlx::Error> {
        sqlx::query_as!(
            Player,
            r#"
                UPDATE players
                SET name = $2, score = $3
                WHERE id = $1
                RETURNING id, name, score, created_at
            "#,
            id,
            name,
            score
        )
        .fetch_one(pool)
        .await
    }
    // Delete a player from the database and don't return anything.
    pub async fn delete(pool: &Pool<Postgres>, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM players WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
