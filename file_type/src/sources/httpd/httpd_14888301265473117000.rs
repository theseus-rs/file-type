use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14888301265473117000: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms xbap",
    extensions: &["xbap"],
    media_types: &["application/x-ms-xbap"],
    internal_signatures: &[],
    related_formats: &[],
};
