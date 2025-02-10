use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_400689417: FileType = FileType {
    file_format: &FileFormat {
        id: 400_689_417,
        source_type: SourceType::Httpd,
        name: "csv",
        extensions: &["csv"],
        media_types: &["text/csv"],
        signatures: &[],
        related_formats: &[],
    },
};
