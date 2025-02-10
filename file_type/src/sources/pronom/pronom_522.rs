use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_522: FileType = FileType {
    file_format: &FileFormat {
        id: 522,
        source_type: SourceType::Pronom,
        name: "SAS for MS-DOS Database",
        extensions: &["ssd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
