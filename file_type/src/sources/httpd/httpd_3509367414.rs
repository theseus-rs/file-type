use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3509367414: FileFormat = FileFormat {
    id: 3_509_367_414,
    source_type: SourceType::Httpd,
    name: "mods xml",
    extensions: &["mods"],
    media_types: &["application/mods+xml"],
    signatures: &[],
    related_formats: &[],
};
