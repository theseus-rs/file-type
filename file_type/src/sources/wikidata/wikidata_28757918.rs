use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28757918: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_918,
        source_type: SourceType::Wikidata,
        name: "Google Sheet",
        extensions: &["gsheet"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
