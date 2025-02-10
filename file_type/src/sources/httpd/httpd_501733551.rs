use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_501733551: FileType = FileType {
    file_format: &FileFormat {
        id: 501_733_551,
        source_type: SourceType::Httpd,
        name: "svd",
        extensions: &["svd"],
        media_types: &["application/vnd.svd"],
        signatures: &[],
        related_formats: &[],
    },
};
