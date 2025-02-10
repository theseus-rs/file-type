use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1462109934: FileType = FileType {
    file_format: &FileFormat {
        id: 1_462_109_934,
        source_type: SourceType::Httpd,
        name: "mbox",
        extensions: &["mbox"],
        media_types: &["application/mbox"],
        signatures: &[],
        related_formats: &[],
    },
};
