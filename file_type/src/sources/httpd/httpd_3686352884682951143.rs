use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3686352884682951143: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dtbresource xml",
    extensions: &["res"],
    media_types: &["application/x-dtbresource+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
