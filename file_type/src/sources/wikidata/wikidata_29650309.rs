use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29650309: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_309,
        source_type: SourceType::Wikidata,
        name: "PQA",
        extensions: &["pqa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
