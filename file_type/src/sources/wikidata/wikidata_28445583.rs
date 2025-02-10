use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28445583: FileType = FileType {
    file_format: &FileFormat {
        id: 28_445_583,
        source_type: SourceType::Wikidata,
        name: "Application Label Cache",
        extensions: &["axc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
