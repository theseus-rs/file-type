use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1485532216: FileType = FileType {
    file_format: &FileFormat {
        id: 1_485_532_216,
        source_type: SourceType::Iana,
        name: "prs.cww",
        extensions: &[],
        media_types: &["application/prs.cww"],
        signatures: &[],
        related_formats: &[],
    },
};
