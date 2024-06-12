pub async fn health() -> axum::response::Response {
    service::response::success("Server is working fine, you can send the traffic")
}

pub async fn create(
    axum::Json(req): axum::Json<service::apis::create_question::CreateReq>,
) -> axum::response::Response {
    match service::apis::create_question::create(req).await {
        Ok(r) => service::response::success(r),
        Err(err) => service::response::error(
            err.to_string(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

pub async fn list() -> axum::response::Response {
    match service::apis::list_questions::list().await {
        Ok(r) => service::response::success(r),
        Err(err) => service::response::error(
            err.to_string(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}
