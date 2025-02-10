use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_462488745: FileType = FileType {
    file_format: &FileFormat {
        id: 462_488_745,
        source_type: SourceType::Linguist,
        name: "mcfunction",
        extensions: &["mcfunction"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
