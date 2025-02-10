use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_437: FileType = FileType {
    file_format: &FileFormat {
        id: 437,
        source_type: SourceType::Pronom,
        name: "DEC Data Exchange File",
        extensions: &["dx"],
        media_types: &["application/dec-dx."],
        signatures: &[],
        related_formats: &[],
    },
};
