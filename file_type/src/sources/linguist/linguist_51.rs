use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_51: FileType = FileType {
    file_format: &FileFormat {
        id: 51,
        source_type: SourceType::Linguist,
        name: "CSV",
        extensions: &["csv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
