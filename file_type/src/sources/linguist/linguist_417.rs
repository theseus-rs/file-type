use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_417: FileType = FileType {
    file_format: &FileFormat {
        id: 417,
        source_type: SourceType::Linguist,
        name: "nesC",
        extensions: &["nc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
