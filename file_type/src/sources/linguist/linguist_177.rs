use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_177: FileType = FileType {
    file_format: &FileFormat {
        id: 177,
        source_type: SourceType::Linguist,
        name: "JSONiq",
        extensions: &["jq"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
