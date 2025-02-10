use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_697448245: FileType = FileType {
    file_format: &FileFormat {
        id: 697_448_245,
        source_type: SourceType::Linguist,
        name: "Q#",
        extensions: &["qs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
