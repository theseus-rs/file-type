use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14018862107975690802: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "nokia radio preset",
    extensions: &["rpst"],
    media_types: &["application/vnd.nokia.radio-preset"],
    internal_signatures: &[],
    related_formats: &[],
};
