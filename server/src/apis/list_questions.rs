use sqlx::Row;

#[derive(Debug, serde::Serialize)]
pub struct Question {
    username: String,
    question: String,
    vote: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list(ctx: &service::Ctx) -> anyhow::Result<Vec<Question>> {
    let query =
        "SELECT username, content, vote_count, created_at from questions ORDER BY vote_count DESC";
    let rows = sqlx::query(query).fetch_all(&ctx.pool).await?;
    let questions: anyhow::Result<Vec<Question>> = rows
        .into_iter()
        .map(|row| -> anyhow::Result<Question> {
            Ok(Question {
                username: row.try_get(0)?,
                question: row.try_get(1)?,
                vote: row.try_get(2)?,
                created_at: row.try_get(3)?,
            })
        })
        .collect();
    Ok(questions?)
}
