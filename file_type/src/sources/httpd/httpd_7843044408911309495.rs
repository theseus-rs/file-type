use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7843044408911309495: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "json",
    extensions: &["json"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
