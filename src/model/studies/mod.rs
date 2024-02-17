pub mod export_one_chapter;

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub type GetRequest = crate::model::Request<GetStudyQuery>;

#[derive(Default, Clone, Debug, Serialize)]
#[skip_serializing_none]
pub struct GetStudyQuery {
    pub clocks: Option<bool>,
    pub comments: Option<bool>,
    pub variations: Option<bool>,
    pub source: Option<bool>,
    pub orientation: Option<bool>
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Study {
    pub chapters: Vec<StudyChapter>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StudyHeader {
    pub header_type: StudyHeaderType,
    pub header_value: String,
}

impl FromStr for StudyHeader {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let s = match value.strip_prefix('[') {
            Some(s) => s,
            None => return Err("Header does not begin with '[' character."),
        };
        let mut s_split = s.split_whitespace();
        let header_type = match s_split.next().unwrap() {
            "Event" => StudyHeaderType::Event,
            "Site" => StudyHeaderType::Site,
            "Result" => StudyHeaderType::Result,
            "Variant" => StudyHeaderType::Variant,
            "ECO" => StudyHeaderType::ECO,
            "Opening" => StudyHeaderType::Opening,
            "Annotator" => StudyHeaderType::Annotator,
            "UTCDate" => StudyHeaderType::UTCDate,
            "UTCTime" => StudyHeaderType::UTCTime,
            "ChapterMode" => StudyHeaderType::ChapterMode,
            _ => StudyHeaderType::Unknown
        };

        let mut header_value = String::new();
        while let Some(next_s) = s_split.next() {
            let mut next_string = next_s.to_string();
            if next_string.starts_with('"') {
                next_string.remove(0);
            }
            if next_string.ends_with(']') {
                next_string.pop();
            }
            if next_string.ends_with('"') {
                next_string.pop();
            }
            header_value += &next_string;
        }

        Ok(StudyHeader {
            header_type: header_type,
            header_value: header_value,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StudyHeaderType {
    Event,
    Site,
    Result,
    Variant,
    ECO,
    Opening,
    Annotator,
    UTCDate,
    UTCTime,
    ChapterMode,
    Unknown
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StudyChapter {
    pub headers: Vec<StudyHeader>,
    pub sections: Vec<StudySection>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StudySection {
    pub comment: String,
    pub pgn: String,
}