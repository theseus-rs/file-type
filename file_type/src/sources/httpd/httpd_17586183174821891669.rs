use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17586183174821891669: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "geometry explorer",
    extensions: &["gex", "gre"],
    media_types: &["application/vnd.geometry-explorer"],
    internal_signatures: &[],
    related_formats: &[],
};
