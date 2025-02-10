use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_169: FileType = FileType {
    file_format: &FileFormat {
        id: 169,
        source_type: SourceType::Linguist,
        name: "Ioke",
        extensions: &["ik"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
