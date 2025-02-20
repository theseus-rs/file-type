use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3174363614: FileType = FileType {
    file_format: &FileFormat {
        id: 3_174_363_614,
        source_type: SourceType::Httpd,
        name: "ms asf",
        extensions: &["asf", "asx"],
        media_types: &["video/x-ms-asf"],
        signatures: &[],
        related_formats: &[],
    },
};
