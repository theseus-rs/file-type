use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_25: FileType = FileType {
    file_format: &FileFormat {
        id: 25,
        source_type: SourceType::Linguist,
        name: "Augeas",
        extensions: &["aug"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
