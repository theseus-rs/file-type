use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_820780064: FileType = FileType {
    file_format: &FileFormat {
        id: 820_780_064,
        source_type: SourceType::Iana,
        name: "vnd.kenameaapp",
        extensions: &[],
        media_types: &["application/vnd.kenameaapp"],
        signatures: &[],
        related_formats: &[],
    },
};
