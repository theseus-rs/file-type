use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2779766884: FileFormat = FileFormat {
    id: 2_779_766_884,
    source_type: SourceType::Httpd,
    name: "xz",
    extensions: &["xz"],
    media_types: &["application/x-xz"],
    internal_signatures: &[],
    related_formats: &[],
};
