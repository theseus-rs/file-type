use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2296385429: FileType = FileType {
    file_format: &FileFormat {
        id: 2_296_385_429,
        source_type: SourceType::Httpd,
        name: "irepository package xml",
        extensions: &["irp"],
        media_types: &["application/vnd.irepository.package+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
