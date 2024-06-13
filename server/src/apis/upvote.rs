pub async fn upvote(ctx: &service::Ctx, id: i64) -> anyhow::Result<()> {
    let query = "UPDATE questions set vote_count = vote_count + 1 where id = $1";
    sqlx::query(query).bind(id).execute(&ctx.pool).await?;
    Ok(())
}
