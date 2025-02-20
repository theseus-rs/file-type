use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2560071141: FileType = FileType {
    file_format: &FileFormat {
        id: 2_560_071_141,
        source_type: SourceType::Iana,
        name: "yin+xml",
        extensions: &[],
        media_types: &["application/yin+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
