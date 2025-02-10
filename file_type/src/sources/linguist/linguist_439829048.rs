use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_439829048: FileType = FileType {
    file_format: &FileFormat {
        id: 439_829_048,
        source_type: SourceType::Linguist,
        name: "Curry",
        extensions: &["curry"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
