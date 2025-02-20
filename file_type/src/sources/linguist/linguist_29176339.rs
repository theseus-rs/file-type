use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_29176339: FileType = FileType {
    file_format: &FileFormat {
        id: 29_176_339,
        source_type: SourceType::Linguist,
        name: "CIL",
        extensions: &["cil"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
