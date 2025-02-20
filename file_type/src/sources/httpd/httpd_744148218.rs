use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_744148218: FileType = FileType {
    file_format: &FileFormat {
        id: 744_148_218,
        source_type: SourceType::Httpd,
        name: "sql",
        extensions: &["sql"],
        media_types: &["application/x-sql"],
        signatures: &[],
        related_formats: &[],
    },
};
