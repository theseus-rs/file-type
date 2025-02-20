use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_530516509: FileType = FileType {
    file_format: &FileFormat {
        id: 530_516_509,
        source_type: SourceType::Iana,
        name: "reputon+json",
        extensions: &[],
        media_types: &["application/reputon+json"],
        signatures: &[],
        related_formats: &[],
    },
};
