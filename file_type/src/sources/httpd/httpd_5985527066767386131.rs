use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5985527066767386131: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "nokia radio presets",
    extensions: &["rpss"],
    media_types: &["application/vnd.nokia.radio-presets"],
    internal_signatures: &[],
    related_formats: &[],
};
