use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3509367414: FileType = FileType {
    file_format: &FileFormat {
        id: 3_509_367_414,
        source_type: SourceType::Iana,
        name: "mods+xml",
        extensions: &[],
        media_types: &["application/mods+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
