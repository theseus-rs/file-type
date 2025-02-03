use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3134068814: FileFormat = FileFormat {
    id: 3_134_068_814,
    source_type: SourceType::Httpd,
    name: "sv4crc",
    extensions: &["sv4crc"],
    media_types: &["application/x-sv4crc"],
    internal_signatures: &[],
    related_formats: &[],
};
