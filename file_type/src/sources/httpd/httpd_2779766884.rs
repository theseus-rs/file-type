use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2779766884: FileType = FileType {
    file_format: &FileFormat {
        id: 2_779_766_884,
        source_type: SourceType::Httpd,
        name: "xz",
        extensions: &["xz"],
        media_types: &["application/x-xz"],
        signatures: &[],
        related_formats: &[],
    },
};
