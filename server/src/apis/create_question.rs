use sqlx::Row;

#[derive(Debug, serde::Deserialize)]
pub struct CreateReq {
    pub username: String,
    pub question: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateResponse {
    id: i64,
}

#[tracing::instrument(skip_all, err)]
pub async fn create(
    ctx: service::Ctx,
    host: &str,
    req: CreateReq,
) -> anyhow::Result<CreateResponse> {
    // call the insert function and return the result

    let created_at = chrono::Utc::now();

    let query = "INSERT INTO questions(username, content, vote_count, host, created_at, updated_at) values($1, $2, $3, $4, $5, $6) returning id";

    let row = sqlx::query(query)
        .bind(req.username.as_str())
        .bind(req.question.as_str())
        .bind(0)
        .bind(host)
        .bind(created_at)
        .bind(created_at)
        .fetch_one(&ctx.pool)
        .await?;

    let id = row.try_get::<i64, _>(0)?;
    println!("Row: {:#?}", id);

    Ok(CreateResponse { id })
}
