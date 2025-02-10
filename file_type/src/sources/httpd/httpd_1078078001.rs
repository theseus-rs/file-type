use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1078078001: FileType = FileType {
    file_format: &FileFormat {
        id: 1_078_078_001,
        source_type: SourceType::Httpd,
        name: "groove help",
        extensions: &["ghf"],
        media_types: &["application/vnd.groove-help"],
        signatures: &[],
        related_formats: &[],
    },
};
