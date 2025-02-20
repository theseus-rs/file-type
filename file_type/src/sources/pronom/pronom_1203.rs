use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1203: FileType = FileType {
    file_format: &FileFormat {
        id: 1_203,
        source_type: SourceType::Pronom,
        name: "Adobe Illustrator",
        extensions: &["ai"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
