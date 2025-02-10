use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3134068814: FileType = FileType {
    file_format: &FileFormat {
        id: 3_134_068_814,
        source_type: SourceType::Httpd,
        name: "sv4crc",
        extensions: &["sv4crc"],
        media_types: &["application/x-sv4crc"],
        signatures: &[],
        related_formats: &[],
    },
};
