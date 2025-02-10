use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1397127342: FileType = FileType {
    file_format: &FileFormat {
        id: 1_397_127_342,
        source_type: SourceType::Iana,
        name: "SGML",
        extensions: &[],
        media_types: &["application/SGML"],
        signatures: &[],
        related_formats: &[],
    },
};
