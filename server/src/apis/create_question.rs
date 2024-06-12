#[derive(Debug, serde::Deserialize)]
pub struct CreateReq {
    pub question: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateResponse {
    id: i64,
}

pub async fn create(req: CreateReq) -> anyhow::Result<CreateResponse> {
    Ok(CreateResponse { id: 1 })
}
