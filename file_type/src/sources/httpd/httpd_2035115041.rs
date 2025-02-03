use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2035115041: FileFormat = FileFormat {
    id: 2_035_115_041,
    source_type: SourceType::Httpd,
    name: "cdmi container",
    extensions: &["cdmic"],
    media_types: &["application/cdmi-container"],
    internal_signatures: &[],
    related_formats: &[],
};
