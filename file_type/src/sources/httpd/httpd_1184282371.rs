use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1184282371: FileFormat = FileFormat {
    id: 1_184_282_371,
    source_type: SourceType::Httpd,
    name: "ccxml xml",
    extensions: &["ccxml"],
    media_types: &["application/ccxml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
