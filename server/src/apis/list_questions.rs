#[derive(Debug, serde::Serialize)]
pub struct QuestionsList {
    question: String,
    vote: i32,
}

pub async fn list() -> anyhow::Result<QuestionsList> {
    Ok(QuestionsList {
        question: "temp".to_string(),
        vote: 1,
    })
}
