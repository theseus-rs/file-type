use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4131086905: FileFormat = FileFormat {
    id: 4_131_086_905,
    source_type: SourceType::Httpd,
    name: "geometry explorer",
    extensions: &["gex", "gre"],
    media_types: &["application/vnd.geometry-explorer"],
    internal_signatures: &[],
    related_formats: &[],
};
