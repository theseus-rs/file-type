use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3000733719: FileType = FileType {
    file_format: &FileFormat {
        id: 3_000_733_719,
        source_type: SourceType::Iana,
        name: "rfc822",
        extensions: &[],
        media_types: &["message/rfc822"],
        signatures: &[],
        related_formats: &[],
    },
};
