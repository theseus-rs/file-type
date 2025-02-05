use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3275176756: FileFormat = FileFormat {
    id: 3_275_176_756,
    source_type: SourceType::Httpd,
    name: "nokia radio presets",
    extensions: &["rpss"],
    media_types: &["application/vnd.nokia.radio-presets"],
    signatures: &[],
    related_formats: &[],
};
