use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_519377561: FileType = FileType {
    file_format: &FileFormat {
        id: 519_377_561,
        source_type: SourceType::Linguist,
        name: "Java Properties",
        extensions: &["properties"],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};
