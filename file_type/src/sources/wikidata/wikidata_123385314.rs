use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123385314: FileType = FileType {
    file_format: &FileFormat {
        id: 123_385_314,
        source_type: SourceType::Wikidata,
        name: "Old material library file",
        extensions: &["mlb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
