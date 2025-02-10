use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_24470517: FileType = FileType {
    file_format: &FileFormat {
        id: 24_470_517,
        source_type: SourceType::Linguist,
        name: "Survex data",
        extensions: &["svx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
