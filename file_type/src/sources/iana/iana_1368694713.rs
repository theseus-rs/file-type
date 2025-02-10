use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1368694713: FileType = FileType {
    file_format: &FileFormat {
        id: 1_368_694_713,
        source_type: SourceType::Iana,
        name: "msword",
        extensions: &[],
        media_types: &["application/msword"],
        signatures: &[],
        related_formats: &[],
    },
};
