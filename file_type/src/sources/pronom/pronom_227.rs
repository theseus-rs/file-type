use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
