use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_87: FileType = FileType {
    file_format: &FileFormat {
        id: 87,
        source_type: SourceType::Linguist,
        name: "Dart",
        extensions: &["dart"],
        media_types: &["application/dart"],
        signatures: &[],
        related_formats: &[],
    },
};
