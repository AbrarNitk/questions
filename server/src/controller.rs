pub async fn health() -> axum::response::Response {
    service::response::success("Server is working fine, you can send the traffic")
}

pub async fn create(
    axum::extract::State(ctx): axum::extract::State<service::Ctx>,
    axum::extract::Host(host): axum::extract::Host,
    axum::Json(req): axum::Json<service::apis::create_question::CreateReq>,
) -> axum::response::Response {
    match service::apis::create_question::create(ctx, host.as_str(), req).await {
        Ok(r) => service::response::success(r),
        Err(err) => service::response::error(
            err.to_string(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

pub async fn upvote(
    axum::extract::State(ctx): axum::extract::State<service::Ctx>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> axum::response::Response {
    match service::apis::upvote::upvote(&ctx, id).await {
        Ok(r) => service::response::success(r),
        Err(err) => service::response::error(
            err.to_string(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

pub async fn list(
    axum::extract::State(ctx): axum::extract::State<service::Ctx>,
) -> axum::response::Response {
    match service::apis::list_questions::list(&ctx).await {
        Ok(r) => service::response::success(r),
        Err(err) => service::response::error(
            err.to_string(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}
