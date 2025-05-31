use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1238387917: FileType = FileType {
    file_format: &FileFormat {
        id: 1_238_387_917,
        source_type: SourceType::Iana,
        name: "sd-jwt+json",
        extensions: &[],
        media_types: &["application/sd-jwt+json"],
        signatures: &[],
        related_formats: &[],
    },
};
