use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_23: FileType = FileType {
    file_format: &FileFormat {
        id: 23,
        source_type: SourceType::Linguist,
        name: "AspectJ",
        extensions: &["aj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
