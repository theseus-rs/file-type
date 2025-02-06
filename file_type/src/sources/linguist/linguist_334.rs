use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_334: FileFormat = FileFormat {
    id: 334,
    source_type: SourceType::Linguist,
    name: "SQLPL",
    extensions: &["db2", "sql"],
    media_types: &["text/x-sql"],
    signatures: &[],
    related_formats: &[],
};
