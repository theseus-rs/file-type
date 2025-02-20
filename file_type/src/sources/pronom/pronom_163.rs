use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_163: FileType = FileType {
    file_format: &FileFormat {
        id: 163,
        source_type: SourceType::Pronom,
        name: "Plain Text File",
        extensions: &["txt"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
