use axum::response::IntoResponse;

#[derive(serde::Serialize, Debug)]
pub struct ApiSuccess<T> {
    success: bool,
    data: T,
}

pub fn success<D: serde::Serialize>(body: D) -> axum::response::Response {
    success_with_status(body, axum::http::StatusCode::OK)
}

pub fn success_with_status<D: serde::Serialize>(
    body: D,
    status: axum::http::StatusCode,
) -> axum::response::Response {
    (
        status,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        axum::Json(ApiSuccess {
            success: true,
            data: body,
        }),
    )
        .into_response()
}

#[derive(serde::Serialize, Debug)]
pub struct ApiError {
    success: bool,
    msg: String,
}

pub fn error(message: &str, status_code: axum::http::StatusCode) -> axum::response::Response {
    (
        status_code,
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        axum::Json(ApiError {
            success: true,
            msg: message.to_string(),
        }),
    )
        .into_response()
}
