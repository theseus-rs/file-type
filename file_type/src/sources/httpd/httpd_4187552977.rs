use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4187552977: FileType = FileType {
    file_format: &FileFormat {
        id: 4_187_552_977,
        source_type: SourceType::Httpd,
        name: "atomcat xml",
        extensions: &["atomcat"],
        media_types: &["application/atomcat+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
