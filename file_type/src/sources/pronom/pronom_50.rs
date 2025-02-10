use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_50: FileType = FileType {
    file_format: &FileFormat {
        id: 50,
        source_type: SourceType::Pronom,
        name: "7-bit ANSI Text",
        extensions: &["ans"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
