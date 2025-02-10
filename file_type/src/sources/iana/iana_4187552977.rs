use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4187552977: FileType = FileType {
    file_format: &FileFormat {
        id: 4_187_552_977,
        source_type: SourceType::Iana,
        name: "atomcat+xml",
        extensions: &[],
        media_types: &["application/atomcat+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
