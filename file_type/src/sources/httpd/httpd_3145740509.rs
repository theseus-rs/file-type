use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3145740509: FileType = FileType {
    file_format: &FileFormat {
        id: 3_145_740_509,
        source_type: SourceType::Httpd,
        name: "umajin",
        extensions: &["umj"],
        media_types: &["application/vnd.umajin"],
        signatures: &[],
        related_formats: &[],
    },
};
