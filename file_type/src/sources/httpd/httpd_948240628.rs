use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_948240628: FileType = FileType {
    file_format: &FileFormat {
        id: 948_240_628,
        source_type: SourceType::Httpd,
        name: "mpeg",
        extensions: &["mpeg", "mpg", "mpe", "m1v", "m2v"],
        media_types: &["video/mpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
