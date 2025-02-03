use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2939067479: FileFormat = FileFormat {
    id: 2_939_067_479,
    source_type: SourceType::Httpd,
    name: "gml xml",
    extensions: &["gml"],
    media_types: &["application/gml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
