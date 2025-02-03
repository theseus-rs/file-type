use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12753032465493873976: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rss xml",
    extensions: &["rss"],
    media_types: &["application/rss+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
