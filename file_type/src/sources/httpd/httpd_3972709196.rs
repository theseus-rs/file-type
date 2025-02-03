use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3972709196: FileFormat = FileFormat {
    id: 3_972_709_196,
    source_type: SourceType::Httpd,
    name: "adobe xdp xml",
    extensions: &["xdp"],
    media_types: &["application/vnd.adobe.xdp+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
