use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2560071141: FileFormat = FileFormat {
    id: 2_560_071_141,
    source_type: SourceType::Httpd,
    name: "yin xml",
    extensions: &["yin"],
    media_types: &["application/yin+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
