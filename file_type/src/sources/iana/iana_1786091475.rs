use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1786091475: FileType = FileType {
    file_format: &FileFormat {
        id: 1_786_091_475,
        source_type: SourceType::Iana,
        name: "CelB",
        extensions: &[],
        media_types: &["video/CelB"],
        signatures: &[],
        related_formats: &[],
    },
};
