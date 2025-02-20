use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_1: FileType = FileType {
    file_format: &FileFormat {
        id: 1,
        source_type: SourceType::Linguist,
        name: "ABAP",
        extensions: &["abap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
