use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2487060900: FileType = FileType {
    file_format: &FileFormat {
        id: 2_487_060_900,
        source_type: SourceType::Httpd,
        name: "shar",
        extensions: &["shar"],
        media_types: &["application/x-shar"],
        signatures: &[],
        related_formats: &[],
    },
};
