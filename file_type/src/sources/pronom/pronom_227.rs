use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_227: FileType = FileType {
    file_format: &FileFormat {
        id: 227,
        source_type: SourceType::Pronom,
        name: "Java Servlet Page",
        extensions: &["jsp"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
