use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_123: FileType = FileType {
    file_format: &FileFormat {
        id: 123,
        source_type: SourceType::Linguist,
        name: "GDScript",
        extensions: &["gd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
