use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1322499123: FileType = FileType {
    file_format: &FileFormat {
        id: 1_322_499_123,
        source_type: SourceType::Httpd,
        name: "xenc xml",
        extensions: &["xenc"],
        media_types: &["application/xenc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
