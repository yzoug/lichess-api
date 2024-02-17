use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::model::{Body, Request};

use crate::model::VariantKey;

#[derive(Default, Clone, Debug, Serialize)]
#[skip_serializing_none]
pub struct ImportPgnBody {
    pub name: String,
    pub pgn: String,
    pub variant: Option<VariantKey>,
    pub orientation: Option<String>,
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct StudyImportPgnChapters {
    pub chapters: Vec<StudyChapterListItem>,
}

#[derive(Default, Clone, Debug, Deserialize)]
pub struct StudyChapterListItem {
    pub id: String,
    pub name: String,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PostQuery;

pub type PostRequest = Request<PostQuery, ImportPgnBody>;

impl PostRequest {
    pub fn new(study_id: String, import_pgn_body: ImportPgnBody) -> Self {
        Self {
            method: http::Method::POST,
            path: format!("/api/study/{}/import-pgn", study_id),
            body: Body::Form(import_pgn_body),
            ..Default::default()
        }
    }
}
