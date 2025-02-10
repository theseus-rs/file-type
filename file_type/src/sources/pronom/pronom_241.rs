use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_241: FileType = FileType {
    file_format: &FileFormat {
        id: 241,
        source_type: SourceType::Pronom,
        name: "PHP Script Page",
        extensions: &["php"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
