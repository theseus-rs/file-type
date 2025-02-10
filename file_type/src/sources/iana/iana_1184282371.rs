use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1184282371: FileType = FileType {
    file_format: &FileFormat {
        id: 1_184_282_371,
        source_type: SourceType::Iana,
        name: "ccxml+xml",
        extensions: &[],
        media_types: &["application/ccxml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
