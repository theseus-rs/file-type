use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_42: FileType = FileType {
    file_format: &FileFormat {
        id: 42,
        source_type: SourceType::Pronom,
        name: "MS-DOS Text File",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
