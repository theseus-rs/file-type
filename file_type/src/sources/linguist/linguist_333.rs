use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_333: FileFormat = FileFormat {
    id: 333,
    source_type: SourceType::Linguist,
    name: "SQL",
    extensions: &[
        "cql", "ddl", "inc", "mysql", "prc", "sql", "tab", "udf", "viw",
    ],
    media_types: &["text/x-sql"],
    internal_signatures: &[],
    related_formats: &[],
};
