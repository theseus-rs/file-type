use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_115: FileType = FileType {
    file_format: &FileFormat {
        id: 115,
        source_type: SourceType::Linguist,
        name: "FreeMarker",
        extensions: &["ftl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
