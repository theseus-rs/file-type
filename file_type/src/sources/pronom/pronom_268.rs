use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_268: FileType = FileType {
    file_format: &FileFormat {
        id: 268,
        source_type: SourceType::Pronom,
        name: "Standard Generalized Markup Language",
        extensions: &["sgml", "sgm"],
        media_types: &["text/sgml"],
        signatures: &[],
        related_formats: &[],
    },
};
