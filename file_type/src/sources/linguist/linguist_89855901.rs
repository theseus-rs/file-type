use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_89855901: FileType = FileType {
    file_format: &FileFormat {
        id: 89_855_901,
        source_type: SourceType::Linguist,
        name: "StringTemplate",
        extensions: &["st"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
