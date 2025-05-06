use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1029438153: FileType = FileType {
    file_format: &FileFormat {
        id: 1_029_438_153,
        source_type: SourceType::Linguist,
        name: "Sail",
        extensions: &["sail"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
