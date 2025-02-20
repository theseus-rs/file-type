use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1568461348: FileType = FileType {
    file_format: &FileFormat {
        id: 1_568_461_348,
        source_type: SourceType::Httpd,
        name: "data vision rdz",
        extensions: &["rdz"],
        media_types: &["application/vnd.data-vision.rdz"],
        signatures: &[],
        related_formats: &[],
    },
};
