use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1780460061: FileType = FileType {
    file_format: &FileFormat {
        id: 1_780_460_061,
        source_type: SourceType::Iana,
        name: "mmt-usd+xml",
        extensions: &[],
        media_types: &["application/mmt-usd+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
