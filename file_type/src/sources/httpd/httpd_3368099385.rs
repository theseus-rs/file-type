use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3368099385: FileType = FileType {
    file_format: &FileFormat {
        id: 3_368_099_385,
        source_type: SourceType::Httpd,
        name: "mathml xml",
        extensions: &["mathml"],
        media_types: &["application/mathml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
