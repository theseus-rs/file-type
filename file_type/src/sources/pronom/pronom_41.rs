use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_41: FileType = FileType {
    file_format: &FileFormat {
        id: 41,
        source_type: SourceType::Pronom,
        name: "Macintosh Text File",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
