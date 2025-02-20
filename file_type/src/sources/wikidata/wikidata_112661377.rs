use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112661377: FileType = FileType {
    file_format: &FileFormat {
        id: 112_661_377,
        source_type: SourceType::Wikidata,
        name: "VIZ Material XML Import",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
