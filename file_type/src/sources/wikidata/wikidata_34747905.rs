use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34747905: FileType = FileType {
    file_format: &FileFormat {
        id: 34_747_905,
        source_type: SourceType::Wikidata,
        name: "Swift script",
        extensions: &["swift"],
        media_types: &["text/x-swift"],
        signatures: &[],
        related_formats: &[],
    },
};
