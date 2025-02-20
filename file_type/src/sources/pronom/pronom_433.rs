use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_433: FileType = FileType {
    file_format: &FileFormat {
        id: 433,
        source_type: SourceType::Pronom,
        name: "8-bit ANSI Text",
        extensions: &["ans"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
