use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_602521492: FileType = FileType {
    file_format: &FileFormat {
        id: 602_521_492,
        source_type: SourceType::Httpd,
        name: "tei xml",
        extensions: &["tei", "teicorpus"],
        media_types: &["application/tei+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
