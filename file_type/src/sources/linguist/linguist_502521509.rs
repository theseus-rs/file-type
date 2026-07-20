use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_502521509: FileType = FileType {
    file_format: &FileFormat {
        id: 502_521_509,
        source_type: SourceType::Linguist,
        name: "BAML",
        extensions: &["baml"],
        media_types: &["text/x-styl"],
        signatures: &[],
        related_formats: &[],
    },
};
