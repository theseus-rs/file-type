use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_602521492: FileType = FileType {
    file_format: &FileFormat {
        id: 602_521_492,
        source_type: SourceType::Iana,
        name: "tei+xml",
        extensions: &[],
        media_types: &["application/tei+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
