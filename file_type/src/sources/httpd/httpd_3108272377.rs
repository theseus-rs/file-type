use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3108272377: FileFormat = FileFormat {
    id: 3_108_272_377,
    source_type: SourceType::Httpd,
    name: "xyz",
    extensions: &["xyz"],
    media_types: &["chemical/x-xyz"],
    internal_signatures: &[],
    related_formats: &[],
};
