use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1125893206: FileFormat = FileFormat {
    id: 1_125_893_206,
    source_type: SourceType::Httpd,
    name: "ms xbap",
    extensions: &["xbap"],
    media_types: &["application/x-ms-xbap"],
    internal_signatures: &[],
    related_formats: &[],
};
