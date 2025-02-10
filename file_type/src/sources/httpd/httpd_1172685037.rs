use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1172685037: FileType = FileType {
    file_format: &FileFormat {
        id: 1_172_685_037,
        source_type: SourceType::Httpd,
        name: "sv4cpio",
        extensions: &["sv4cpio"],
        media_types: &["application/x-sv4cpio"],
        signatures: &[],
        related_formats: &[],
    },
};
