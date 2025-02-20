use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_267: FileType = FileType {
    file_format: &FileFormat {
        id: 267,
        source_type: SourceType::Linguist,
        name: "Org",
        extensions: &["org"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
