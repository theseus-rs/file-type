use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1204394635: FileType = FileType {
    file_format: &FileFormat {
        id: 1_204_394_635,
        source_type: SourceType::Httpd,
        name: "ms fontobject",
        extensions: &["eot"],
        media_types: &["application/vnd.ms-fontobject"],
        signatures: &[],
        related_formats: &[],
    },
};
