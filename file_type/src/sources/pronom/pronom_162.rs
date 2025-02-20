use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_162: FileType = FileType {
    file_format: &FileFormat {
        id: 162,
        source_type: SourceType::Pronom,
        name: "Fixed Width Values Text File",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
