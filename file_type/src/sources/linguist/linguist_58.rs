use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_58: FileType = FileType {
    file_format: &FileFormat {
        id: 58,
        source_type: SourceType::Linguist,
        name: "Cirru",
        extensions: &["cirru"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
