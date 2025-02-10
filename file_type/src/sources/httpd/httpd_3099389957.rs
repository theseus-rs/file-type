use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3099389957: FileType = FileType {
    file_format: &FileFormat {
        id: 3_099_389_957,
        source_type: SourceType::Httpd,
        name: "abiword",
        extensions: &["abw"],
        media_types: &["application/x-abiword"],
        signatures: &[],
        related_formats: &[],
    },
};
