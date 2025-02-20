use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2231888978: FileType = FileType {
    file_format: &FileFormat {
        id: 2_231_888_978,
        source_type: SourceType::Httpd,
        name: "gca compressed",
        extensions: &["gca"],
        media_types: &["application/x-gca-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
