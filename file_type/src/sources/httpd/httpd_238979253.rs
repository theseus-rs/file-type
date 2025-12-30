use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_238979253: FileType = FileType {
    file_format: &FileFormat {
        id: 238_979_253,
        source_type: SourceType::Httpd,
        name: "sqlite3",
        extensions: &["sqlite", "sqlite3"],
        media_types: &["application/vnd.sqlite3"],
        signatures: &[],
        related_formats: &[],
    },
};
