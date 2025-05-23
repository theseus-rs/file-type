use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1993520449: FileType = FileType {
    file_format: &FileFormat {
        id: 1_993_520_449,
        source_type: SourceType::Httpd,
        name: "sgi",
        extensions: &["sgi"],
        media_types: &["image/sgi"],
        signatures: &[],
        related_formats: &[],
    },
};
