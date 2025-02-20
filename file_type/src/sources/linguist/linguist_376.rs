use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_376: FileType = FileType {
    file_format: &FileFormat {
        id: 376,
        source_type: SourceType::Linguist,
        name: "Turtle",
        extensions: &["ttl"],
        media_types: &["text/turtle"],
        signatures: &[],
        related_formats: &[],
    },
};
