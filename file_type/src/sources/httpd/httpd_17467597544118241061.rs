use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17467597544118241061: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sv4crc",
    extensions: &["sv4crc"],
    media_types: &["application/x-sv4crc"],
    internal_signatures: &[],
    related_formats: &[],
};
