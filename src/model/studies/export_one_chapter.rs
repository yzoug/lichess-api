use super::GetStudyQuery;

pub type GetRequest = crate::model::Request<GetStudyQuery>;

impl GetRequest {
    pub fn new(study_id: String, chapter_id: String, query: GetStudyQuery) -> Self {
        Self {
            path: format!("/api/study/{}/{}.pgn", study_id, chapter_id),
            query: Some(query),
            ..Default::default()
        }
    }
}
