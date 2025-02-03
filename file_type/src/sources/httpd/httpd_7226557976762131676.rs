use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7226557976762131676: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sv4cpio",
    extensions: &["sv4cpio"],
    media_types: &["application/x-sv4cpio"],
    internal_signatures: &[],
    related_formats: &[],
};
