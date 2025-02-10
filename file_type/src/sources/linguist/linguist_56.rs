use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_56: FileType = FileType {
    file_format: &FileFormat {
        id: 56,
        source_type: SourceType::Linguist,
        name: "Charity",
        extensions: &["ch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
