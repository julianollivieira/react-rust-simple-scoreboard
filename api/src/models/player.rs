use sqlx::types::Uuid;

struct Player {
    id: Uuid,
    name: String,
    score: i32,
    created_at: NaiveDateTime,
}
