use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_932: FileType = FileType {
    file_format: &FileFormat {
        id: 932,
        source_type: SourceType::Pronom,
        name: "Structured Query Language Data",
        extensions: &["sql"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
