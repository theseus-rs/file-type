use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2179273523: FileType = FileType {
    file_format: &FileFormat {
        id: 2_179_273_523,
        source_type: SourceType::Httpd,
        name: "hal xml",
        extensions: &["hal"],
        media_types: &["application/vnd.hal+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
