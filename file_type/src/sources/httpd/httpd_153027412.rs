use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_153027412: FileType = FileType {
    file_format: &FileFormat {
        id: 153_027_412,
        source_type: SourceType::Httpd,
        name: "msvideo",
        extensions: &["avi"],
        media_types: &["video/x-msvideo"],
        signatures: &[],
        related_formats: &[],
    },
};
