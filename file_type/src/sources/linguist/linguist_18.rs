use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_18: FileType = FileType {
    file_format: &FileFormat {
        id: 18,
        source_type: SourceType::Linguist,
        name: "Apollo Guidance Computer",
        extensions: &["agc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
