use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_79411734: FileFormat = FileFormat {
    id: 79_411_734,
    source_type: SourceType::Httpd,
    name: "svg xml",
    extensions: &["svg", "svgz"],
    media_types: &["image/svg+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
