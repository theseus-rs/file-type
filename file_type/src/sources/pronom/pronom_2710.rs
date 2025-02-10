use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2710: FileType = FileType {
    file_format: &FileFormat {
        id: 2_710,
        source_type: SourceType::Pronom,
        name: "Enhanced Image Package",
        extensions: &["eip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
