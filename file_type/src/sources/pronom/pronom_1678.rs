use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1678: FileType = FileType {
    file_format: &FileFormat {
        id: 1_678,
        source_type: SourceType::Pronom,
        name: "Turtle",
        extensions: &["ttl"],
        media_types: &["text/turtle"],
        signatures: &[],
        related_formats: &[],
    },
};
