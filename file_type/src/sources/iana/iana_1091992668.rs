use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1091992668: FileType = FileType {
    file_format: &FileFormat {
        id: 1_091_992_668,
        source_type: SourceType::Iana,
        name: "vnd.nuera.ecelp9600",
        extensions: &[],
        media_types: &["audio/vnd.nuera.ecelp9600"],
        signatures: &[],
        related_formats: &[],
    },
};
