pub async fn health_router() -> axum::Router {
    axum::Router::new().route(
        "/v1/api/health/",
        axum::routing::get(crate::controller::health),
    )
}

pub async fn api_router() -> axum::Router {
    axum::Router::new()
        .route(
            "/v1/api/question/",
            axum::routing::post(crate::controller::question_post),
        )
        .route(
            "/v1/api/questions/",
            axum::routing::get(crate::controller::question_list),
        )
}

pub async fn routes() -> axum::Router {
    axum::Router::new()
        .merge(health_router().await)
        .merge(api_router().await)
}
