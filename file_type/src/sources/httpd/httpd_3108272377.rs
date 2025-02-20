use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3108272377: FileType = FileType {
    file_format: &FileFormat {
        id: 3_108_272_377,
        source_type: SourceType::Httpd,
        name: "xyz",
        extensions: &["xyz"],
        media_types: &["chemical/x-xyz"],
        signatures: &[],
        related_formats: &[],
    },
};
