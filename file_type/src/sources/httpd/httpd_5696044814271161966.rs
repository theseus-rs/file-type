use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5696044814271161966: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sql",
    extensions: &["sql"],
    media_types: &["application/x-sql"],
    internal_signatures: &[],
    related_formats: &[],
};
