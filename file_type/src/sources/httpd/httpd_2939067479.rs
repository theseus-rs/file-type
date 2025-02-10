use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2939067479: FileType = FileType {
    file_format: &FileFormat {
        id: 2_939_067_479,
        source_type: SourceType::Httpd,
        name: "gml xml",
        extensions: &["gml"],
        media_types: &["application/gml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
