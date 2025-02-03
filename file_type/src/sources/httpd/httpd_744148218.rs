use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_744148218: FileFormat = FileFormat {
    id: 744_148_218,
    source_type: SourceType::Httpd,
    name: "sql",
    extensions: &["sql"],
    media_types: &["application/x-sql"],
    internal_signatures: &[],
    related_formats: &[],
};
