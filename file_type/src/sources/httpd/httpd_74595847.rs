use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_74595847: FileType = FileType {
    file_format: &FileFormat {
        id: 74_595_847,
        source_type: SourceType::Httpd,
        name: "bmp",
        extensions: &["bmp"],
        media_types: &["image/bmp"],
        signatures: &[],
        related_formats: &[],
    },
};
