use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3227546503: FileType = FileType {
    file_format: &FileFormat {
        id: 3_227_546_503,
        source_type: SourceType::Httpd,
        name: "shockwave flash",
        extensions: &["swf"],
        media_types: &["application/x-shockwave-flash"],
        signatures: &[],
        related_formats: &[],
    },
};
