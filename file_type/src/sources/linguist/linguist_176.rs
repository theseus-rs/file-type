use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_176: FileType = FileType {
    file_format: &FileFormat {
        id: 176,
        source_type: SourceType::Linguist,
        name: "JSONLD",
        extensions: &["jsonld"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
