use std::str::FromStr;

use async_std::stream::StreamExt;
use tracing::error;

use crate::client::LichessApi;
use crate::error::Result;
use crate::model::studies::*;

impl LichessApi<reqwest::Client> {
    pub async fn export_one_chapter(
        &self,
        request: GetRequest,
    ) -> Result<StudyChapter> {
        let mut stream = self.get_pgn(request).await?;
        let mut study_chapter = StudyChapter {
            headers: vec![],
            sections: vec![],
        };

        let mut current_section = StudySection::default();
        let mut inside_comment = false;
        let mut switch_section_next = false;

        while let Some(s) = stream.next().await {
            let line = s?;
            println!("Got line: {}", line);
            if line.starts_with('[') {
                match StudyHeader::from_str(&line) {
                    Ok(h) => study_chapter.headers.push(h),
                    Err(e) => error!("Can't parse study header: {e}. Ignoring line '{line}'."),
                }
            } else {
                for c in line.chars() {
                    if c == '{' || c == '}' {
                        inside_comment = if c == '{' { true } else { false };
                        if switch_section_next {
                            study_chapter.sections.push(current_section);
                            current_section = StudySection::default();
                            switch_section_next = false;
                        } else {
                            switch_section_next = true;
                        }
                    } else if inside_comment  {
                        current_section.comment.push(c);
                    } else {
                        current_section.pgn.push(c);
                    }
                }
                if inside_comment {
                    current_section.comment.push('\n');
                } else {
                    current_section.pgn.push('\n');
                }
            }
        }

        Ok(study_chapter)
    }
}
