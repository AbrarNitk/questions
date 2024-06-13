pub async fn health_router<S: Send + Sync + Clone + 'static>() -> axum::Router<S> {
    axum::Router::new().route(
        "/v1/api/health/",
        axum::routing::get(crate::controller::health),
    )
}

pub async fn api_router<S: Send + Sync>(ctx: service::Ctx) -> axum::Router<S> {
    axum::Router::new()
        .route(
            "/v1/api/question/",
            axum::routing::post(crate::controller::create),
        )
        .route(
            "/v1/api/questions/",
            axum::routing::get(crate::controller::list),
        )
        .with_state(ctx)
}

pub async fn routes<S: Send + Sync + Clone + 'static>(ctx: service::Ctx) -> axum::Router<S> {
    axum::Router::new()
        .merge(health_router().await)
        .merge(api_router(ctx).await)
}
