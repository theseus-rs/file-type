use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_341496207: FileType = FileType {
    file_format: &FileFormat {
        id: 341_496_207,
        source_type: SourceType::Httpd,
        name: "ecmascript",
        extensions: &["ecma"],
        media_types: &["application/ecmascript"],
        signatures: &[],
        related_formats: &[],
    },
};
