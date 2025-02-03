use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3490122414949210874: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "woff2",
    extensions: &["woff2"],
    media_types: &["font/woff2"],
    internal_signatures: &[],
    related_formats: &[],
};
