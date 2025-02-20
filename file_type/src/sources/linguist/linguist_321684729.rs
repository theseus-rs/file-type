use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_321684729: FileType = FileType {
    file_format: &FileFormat {
        id: 321_684_729,
        source_type: SourceType::Linguist,
        name: "CODEOWNERS",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
