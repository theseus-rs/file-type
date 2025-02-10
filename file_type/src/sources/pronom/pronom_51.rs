use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_51: FileType = FileType {
    file_format: &FileFormat {
        id: 51,
        source_type: SourceType::Pronom,
        name: "7-bit ASCII Text",
        extensions: &["asc"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
