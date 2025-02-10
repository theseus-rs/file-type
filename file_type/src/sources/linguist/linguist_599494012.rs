use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_599494012: FileType = FileType {
    file_format: &FileFormat {
        id: 599_494_012,
        source_type: SourceType::Linguist,
        name: "Java Template Engine",
        extensions: &["jte"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
