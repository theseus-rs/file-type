use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16843356083413255425: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "scvp vp request",
    extensions: &["spq"],
    media_types: &["application/scvp-vp-request"],
    internal_signatures: &[],
    related_formats: &[],
};
