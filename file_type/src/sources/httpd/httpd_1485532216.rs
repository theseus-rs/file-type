use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1485532216: FileType = FileType {
    file_format: &FileFormat {
        id: 1_485_532_216,
        source_type: SourceType::Httpd,
        name: "prs cww",
        extensions: &["cww"],
        media_types: &["application/prs.cww"],
        signatures: &[],
        related_formats: &[],
    },
};
