use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_915515483: FileFormat = FileFormat {
    id: 915_515_483,
    source_type: SourceType::Httpd,
    name: "html",
    extensions: &["html", "htm"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
