use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
