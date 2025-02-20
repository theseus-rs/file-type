use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_334: FileType = FileType {
    file_format: &FileFormat {
        id: 334,
        source_type: SourceType::Linguist,
        name: "SQLPL",
        extensions: &["db2", "sql"],
        media_types: &["text/x-sql"],
        signatures: &[],
        related_formats: &[],
    },
};
