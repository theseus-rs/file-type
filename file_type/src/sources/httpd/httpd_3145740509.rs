use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3145740509: FileFormat = FileFormat {
    id: 3_145_740_509,
    source_type: SourceType::Httpd,
    name: "umajin",
    extensions: &["umj"],
    media_types: &["application/vnd.umajin"],
    internal_signatures: &[],
    related_formats: &[],
};
