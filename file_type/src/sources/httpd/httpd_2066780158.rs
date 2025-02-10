use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2066780158: FileType = FileType {
    file_format: &FileFormat {
        id: 2_066_780_158,
        source_type: SourceType::Httpd,
        name: "cml",
        extensions: &["cml"],
        media_types: &["chemical/x-cml"],
        signatures: &[],
        related_formats: &[],
    },
};
