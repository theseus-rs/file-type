use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1841200561: FileType = FileType {
    file_format: &FileFormat {
        id: 1_841_200_561,
        source_type: SourceType::Httpd,
        name: "sru xml",
        extensions: &["sru"],
        media_types: &["application/sru+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
