use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3710687909: FileType = FileType {
    file_format: &FileFormat {
        id: 3_710_687_909,
        source_type: SourceType::Httpd,
        name: "crick clicker template",
        extensions: &["clkt"],
        media_types: &["application/vnd.crick.clicker.template"],
        signatures: &[],
        related_formats: &[],
    },
};
