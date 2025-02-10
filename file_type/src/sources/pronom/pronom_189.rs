use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_189: FileType = FileType {
    file_format: &FileFormat {
        id: 189,
        source_type: SourceType::Pronom,
        name: "MS-DOS Text File with line breaks",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
